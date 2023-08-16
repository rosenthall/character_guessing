use config::CONFIG;
use database::*;
use log::info;
use teloxide::prelude::*;
use teloxide::Bot;
use tokio::sync::MutexGuard;

use teloxide_macros::BotCommands;

use database::model::UserDbEntry;
use teloxide::types::User as TelergamUser;

use rusqlite::Connection;

#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "Assume today's character.")]
    Answer(String),

    #[command(description = "Make request to gpt4")]
    Gpt(String),

    #[command(description = "View your remaining requests")]
    Requests,

    #[command(description = "Ask a question of today's character.")]
    Question(String),

    #[command(description = "Get a list of today's winners(admins only).")]
    Winners,

    #[command(description = "Just information about the game")]
    Info,
}

pub struct CommandContext<'a> {
    pub db_entry_user: &'a UserDbEntry,
    pub telegram_user: TelergamUser,
    pub msg: Message,
    pub command_content: String,
    pub bot: &'a Bot,
    pub con: MutexGuard<'a, Connection>,
    pub winnersdb_con : MutexGuard<'a, Connection>
}

//noinspection ALL
// Функция проверяет условия комманды и передает выполнение в чат
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
    info!(
        "Group {} is in whitelist : {}",
        msg.chat.clone().id.0,
        is_chat_in_whitelist.clone()
    );

    // Если чат не в вайтлисте - комманду не обрабатываем.
    if !is_chat_in_whitelist {
        return Ok(());
    }

    //Создаем инстансы подключения к базам данных
    let con = control::DATABASE_HANDLER.lock().await;
    let winners_con = database::winners::WINNERS_DB.lock().await;

    //Проверяем есть ли этот пользователь в базе данных
    let author = msg.clone();
    let author = author.from().unwrap();
    let user = check_user(author.id.0, &con);

    let is_user_exists_in_db = user.is_some();
    dbg!(is_user_exists_in_db);

    // Добавляем пользователя в дб, если его там нет
    if !is_user_exists_in_db {
        try_add_user(
            UserDbEntry {
                id: author.id.0,
                attempts: 0,
                is_won: false,
                questions_quantity: 0,
            },
            &con,
        )
        .unwrap()
    }

    let context = CommandContext {
        db_entry_user: &check_user(author.id.0, &con).unwrap(),
        telegram_user: author.clone(),
        msg: msg.clone(),
        command_content: "".to_string(),
        bot: &bot,
        con : con,
        winnersdb_con : winners_con
    };

    match cmd.clone() {
        Command::Answer(cmd) => {
            // Обновляем контекст комманды, изменяя лишь cmd
            let context = CommandContext {
                command_content: cmd,

                ..context
            };

            crate::commands::answer::execute(context).await.unwrap();

            Ok(())
        },

        Command::Question(cmd) => {
            let context = CommandContext {
                command_content: cmd,

                ..context
            };

            crate::commands::question::execute(context).await.unwrap();

            Ok(())
        },

        Command::Gpt(cmd) => {
            let context = CommandContext {
                command_content: cmd,

                ..context
            };

            crate::commands::gpt::execute(context).await.unwrap();

            Ok(())
        },

        Command::Requests => {
            crate::commands::requests::execute(context).await.unwrap();

            Ok(())
        }


        Command::Winners => {
            crate::commands::winners::execute(context).await.unwrap();

            Ok(())
        },

        Command::Info => {
            crate::commands::info::execute(context).await.unwrap();

            Ok(())
        }
    }
}
