use config::CONFIG;
use database::model::User;
use database::*;
use log::{info, trace};
use teloxide::prelude::*;
use teloxide::{Bot};
use teloxide::payloads::{GetChatMember, SendMessageSetters};
use teloxide::requests::JsonRequest;

use teloxide_macros::BotCommands;

#[derive(BotCommands, Clone, Debug)]
#[command(
rename_rule = "lowercase",
description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Assume today's character.")]
    Answer(String),

    #[command(description = "Ask a question of today's character.")]
    Question(String),

    #[command(description = "Get a list of today's winners(admins only).")]
    Winners,

    #[command(description = "Just information about the game")]
    Info
}

//noinspection ALL
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command,) -> ResponseResult<()> {
    info!("Got new command in group : {}", msg.chat.clone().id.0);

    // Проверяем есть ли эта группа в вайтлисте
    let allowed_groups = CONFIG
        .telegram
        .telegram_allowed_groups
        .clone()
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let is_chat_in_whitelist = allowed_groups.contains(&msg.chat.clone().id.0);
    info!("Group {} is in whitelist : {}", msg.chat.clone().id.0, is_chat_in_whitelist.clone());
    //Создаем инстанс подключения к базе данных
    let con = control::DATABASE_HANDLER.lock().await;
    //Проверяем есть ли этот пользователь в базе данных
    let author = msg.from().unwrap();
    let user = check_user(author.id.0, &con);
    let is_user_exists_in_db = user.is_some();

    dbg!(is_user_exists_in_db);
    // Добавляем пользователя в дб, если его там нет

    if !is_user_exists_in_db {
        try_add_user(
            User {
                id: author.id.0,
                attempts: 0,
                is_won: false,
                questions_quantity: 0,
            },
            &con,
        )
            .unwrap()
    }
    // Переинициализируем user, на этот раз с unwrap, ибо он должен существовать на этом этапе
    let user = check_user(author.id.0, &con).unwrap();

    // Если чат не в вайтлисте - комманду не обрабатываем.
    if !is_chat_in_whitelist {
        return Ok(());
    }

    match cmd {
        Command::Answer(cmd) => {
            bot.delete_message(msg.chat.id, msg.id).await.unwrap();

            if user.is_won {
                let _ = bot.send_message(msg.chat.id, "Ты уже победил сегодня!")
                    .reply_to_message_id(msg.id)
                    .await;
            }
            //Если человек попытался угадать больше 5 раз - отказываем.
            if user.attempts >= 5 {
                bot.send_message(
                    msg.chat.id,
                    "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
                ).reply_to_message_id(msg.id)
                    .await
                    .unwrap();
                return Ok(());
            }

            let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();

            if character_names.contains(&cmd) {
                let _ = bot.send_message(
                    msg.chat.id,
                    format!(
                        "Пользователь {} отгадал сегодняшнего персонажа!",
                        author.mention().unwrap_or(author.clone().first_name)
                    ),
                )
                    .await;

                update_is_won(&con, user.id, true).unwrap()
            } else {
                let _ = bot.send_message(msg.chat.id, "Вам не удалось угадать персонажа!")
                    .await;
            };

            Ok(())
        }

        Command::Question(cmd) => {
            info!("New question : {cmd}");

            // Минимальная длинна вопроса. Если вопрос короче 5 символов, то и отвечать смысла нет.
            if cmd.len() <= 5 {
                bot.send_message(
                    msg.chat.id,
                    "После команды \"/question\" должен следовать вопрос. \n Подробнее - /info",
                ).reply_to_message_id(msg.id)
                    .await
                    .unwrap();
                return Ok(());
            }

            //Если человек уже задал больше трех вопросов - отказываем.
            if user.questions_quantity >= 3 {
                bot.send_message(
                    msg.chat.id,
                    "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
                ).reply_to_message_id(msg.id)
                    .await
                    .unwrap();
                return Ok(());
            }

            let ai_answer = openai::question(cmd).await;
            info!("AI ANSWER : {}", ai_answer.clone());

            let _ = bot.send_message(msg.chat.id, ai_answer).reply_to_message_id(msg.id).await;

            //Увеличиваем количество заданных вопросов на 1
            trace!(
                "Увеличенно количество заданных вопросов на 1 для пользователя : {}",
                user.id
            );
            update_questions_quantity(&con, user.id, user.questions_quantity + 1).unwrap();
            Ok(())
        }

        Command::Winners => {

            //If user id is not in admin list - Do nothing
            if !CONFIG.clone().telegram.telegram_admin_ids.contains(&author.id.0.to_string()) {
                trace!("User {} is not in admin list!", &author.id.0.to_string());
                return Ok(());
            }

            let mut message: String = String::from("Вот люди которые справились с угадыванием сегодняшнего персонажа : \n ");
            let winners_list = {
                let mut users = vec!();
                let requests = get_winning_user_ids(&con).unwrap()
                    .iter()
                    .map(|i| UserId(*i))
                    .map(|i| bot.get_chat_member(msg.chat.id, i))
                    .collect::<Vec<JsonRequest<GetChatMember>>>();

                for request in requests {
                    let member = request.await.unwrap();
                    users.push(member.user);
                }

                users
            };


            for winner in winners_list {
                message.insert_str(message.len(),&format!("{} ", &winner.mention().or(Some(winner.first_name)).unwrap()));
            }

            let _ = bot.send_message(msg.chat.id, message).reply_to_message_id(msg.id).await;


            Ok(())
        }
        Command::Info => {
            let _ = bot.send_message(msg.chat.id, "Вот ссылка на мануал : https://telegra.ph/Kak-polzovatsya-botom-08-09").reply_to_message_id(msg.id).await;

            Ok(())
        }


    }
}