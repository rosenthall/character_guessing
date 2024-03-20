use config::CONFIG;
use database::*;
use log::info;
use teloxide::{prelude::*, Bot};
use tokio::sync::MutexGuard;

use teloxide_macros::BotCommands;

use database::model::UserDbEntry;
use teloxide::types::User as TelergamUser;

use rusqlite::Connection;

// Enum for different types of commands
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
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

// Struct for command context
pub struct CommandContext<'a> {
    pub db_entry_user: &'a UserDbEntry,
    pub telegram_user: TelergamUser,
    pub msg: Message,
    pub command_content: String,
    pub bot: &'a Bot,
    pub con: MutexGuard<'a, Connection>,
    pub winnersdb_con: MutexGuard<'a, Connection>,
}

// Handles incoming commands from the Telegram bot.
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    // Log the new command received in the group
    info!("Got new command in group : {}", msg.chat.clone().id.0);

    // Check if the group is in the whitelist
    let allowed_groups = CONFIG
        .telegram
        .telegram_allowed_groups
        .clone()
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let is_chat_in_whitelist = allowed_groups.contains(&msg.chat.clone().id.0);
    // Log the result of the whitelist check
    info!("Group {} is in whitelist : {}", msg.chat.clone().id.0, is_chat_in_whitelist.clone());

    // If the chat is not in the whitelist, do not process the command.
    if !is_chat_in_whitelist {
        return Ok(());
    }

    // Lock the database connections
    let con = control::DATABASE_HANDLER.lock().await;
    let winners_con = database::winners::WINNERS_DB.lock().await;

    // Get the author of the message
    let author = msg.clone();
    let author = author.from().unwrap();
    // Check if the user exists in the database
    let user = check_user(author.id.0, &con);

    let is_user_exists_in_db = user.is_some();
    dbg!(is_user_exists_in_db);

    // If the user does not exist in the database, add them
    if !is_user_exists_in_db {
        try_add_user(
            UserDbEntry { id: author.id.0, attempts: 0, is_won: false, questions_quantity: 0 },
            &con,
        )
        .unwrap()
    }

    // Create a context for the command
    let context = CommandContext {
        db_entry_user: &check_user(author.id.0, &con).unwrap(),
        telegram_user: author.clone(),
        msg: msg.clone(),
        command_content: "".to_string(),
        bot: &bot,
        con,
        winnersdb_con: winners_con,
    };

    // Match the command and execute the corresponding function
    match cmd.clone() {
        Command::Answer(cmd) => {
            // Update the command context, changing only cmd
            let context = CommandContext { command_content: cmd, ..context };

            // Execute the answer command
            crate::commands::answer::execute(context).await.unwrap();

            Ok(())
        }

        Command::Question(cmd) => {
            let context = CommandContext { command_content: cmd, ..context };

            // Execute the question command
            crate::commands::question::execute(context).await.unwrap();

            Ok(())
        }

        Command::Gpt(cmd) => {
            let context = CommandContext { command_content: cmd, ..context };

            // Execute the GPT command
            crate::commands::gpt::execute(context).await.unwrap();

            Ok(())
        }

        Command::Requests => {
            // Execute the requests command
            crate::commands::requests::execute(context).await.unwrap();

            Ok(())
        }

        Command::Winners => {
            // Execute the winners command
            crate::commands::winners::execute(context).await.unwrap();

            Ok(())
        }

        Command::Info => {
            // Execute the info command
            crate::commands::info::execute(context).await.unwrap();

            Ok(())
        }
    }
}
