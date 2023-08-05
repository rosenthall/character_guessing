use config::CONFIG;
use database::model::User;
use database::*;
use log::{info, trace};
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::Bot;
use teloxide_macros::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Assume today's character.")]
    Answer(String),

    #[command(description = "Ask a question of today's character.")]
    Question(String),
}

pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
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

    //Создаем инстанс подключения к базе данных
    let con = database::control::DATABASE.try_get_connection().await;

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
                bot.send_message(msg.chat.id, "Ты уже победил сегодня!")
                    .await;
            }
            //Если человек попытался угадать больше 5 раз - отказываем.
            if user.attempts >= 5 {
                bot.send_message(
                    msg.chat.id,
                    "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
                )
                .await
                .unwrap();
                return Ok(());
            }

            if cmd == CONFIG.calendar.try_get_daily_character().unwrap() {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Да, {}, это именно я, ты победил!",
                        author.mention().unwrap_or(author.clone().first_name)
                    ),
                )
                .await;

                update_is_won(&con, user.id, true).unwrap()
            } else {
                bot.send_message(msg.chat.id, "Вам не удалось угадать персонажа!")
                    .await;
            };

            Ok(())
        }

        Command::Question(cmd) => {
            info!("CMD : {cmd}");

            //Если человек уже задал больше трех вопросов - отказываем.
            if user.questions_quantity >= 3 {
                bot.send_message(
                    msg.chat.id,
                    "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
                )
                .await
                .unwrap();
                return Ok(());
            }

            let ai_answer = openai::question(cmd).await;
            info!("AI ANSWER : {}", ai_answer.clone());

            bot.send_message(msg.chat.id, ai_answer).await;

            //Увеличиваем количество заданных вопросов на 1
            trace!(
                "Увеличенно количество заданных вопросов на 1 для пользователя : {}",
                user.id
            );
            update_questions_quantity(&con, user.id, user.questions_quantity + 1).unwrap();
            Ok(())
        }
    }
}
