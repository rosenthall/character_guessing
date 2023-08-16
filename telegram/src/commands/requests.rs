use crate::command::CommandContext;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use database::model::WinnerEntry;
use database::winners::{try_add_winner, try_get_winner};

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {

    let winner_entry  = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).or_else(|| {

        // Добавляем если такого поля нет.
        try_add_winner(WinnerEntry {
            id: ctx.telegram_user.id.0,
            requests: 0
        }, &ctx.winnersdb_con).unwrap();

        Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
    }).unwrap();



    let _ = ctx
        .bot
        .send_message(
            ctx.msg.chat.id,
            &format!("{}, у тебя осталось {} запросов! Подробнее : /info",
                     ctx.telegram_user.mention()
                         .unwrap_or(ctx.telegram_user.first_name),
                     winner_entry.requests
            ),
        )
        .reply_to_message_id(ctx.msg.id)
        .await;

    Ok(())
}
