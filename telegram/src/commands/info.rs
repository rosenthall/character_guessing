use teloxide::requests::Requester;
use crate::command::{CommandContext};
use teloxide::payloads::{SendMessageSetters};

pub async fn execute(ctx : CommandContext<'_>) -> Result<(), ()> {
    let _ = ctx.bot.send_message(ctx.msg.chat.id, "Вот ссылка на мануал : https://telegra.ph/Kak-polzovatsya-botom-08-09").reply_to_message_id(ctx.msg.id).await;

    Ok(())
}