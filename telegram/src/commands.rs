use log::{info};
use teloxide::Bot;
use teloxide::prelude::{Message, ResponseResult};
use teloxide_macros::BotCommands;
use config::CONFIG;


#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
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
        .telegram.telegram_allowed_groups
        .clone()
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();


    let is_chat_in_whitelist = allowed_groups.contains(&msg.chat.clone().id.0);

    info!("Group is in whitelist : {} .", is_chat_in_whitelist.clone());


    if is_chat_in_whitelist {

        match cmd {
            Command::Answer(_) => {
                todo!();
            }
            Command::Question(_) => {
                todo!();
            }
        }
    } else {
        return Ok(())
    };
}
