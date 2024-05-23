use crate::handler::CommandContext;
use database::{
    model::WinnerEntry,
    winners::{try_add_winner, try_get_winner},
};
use teloxide::{payloads::SendMessageSetters, requests::Requester};

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    // Attempt to get the winner from the database
    let winner_entry = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con)
        .or_else(|| {
            // If the winner does not exist, add a new winner to the database
            try_add_winner(
                WinnerEntry { id: ctx.telegram_user.id.0, requests: 0 },
                &ctx.winnersdb_con,
            )
            .unwrap();

            // Retrieve the newly added winner
            Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
        })
        .unwrap();

    // Send a message to the user indicating the number of requests left
    let _ = ctx
        .bot
        .send_message(
            ctx.msg.chat.id,
            &format!(
                "{}, у тебя осталось {} запросов! Подробнее : /info",
                ctx.telegram_user.mention().unwrap_or(ctx.telegram_user.first_name),
                winner_entry.requests
            ),
        )
        .reply_to_message_id(ctx.msg.id)
        .await;

    // Return Ok to indicate successful execution
    Ok(())
}
