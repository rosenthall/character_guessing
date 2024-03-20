use chrono::Local;
use crate::handler::CommandContext;
use database::model::WinnerEntry;
use database::winners::*;
use log::info;
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New gpt question : {}", ctx.command_content);

    // Attempt to get the winner from the database
    let winner_entry = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con)
        .or_else(|| {
            // If the winner does not exist, add a new winner to the database
            try_add_winner(
                WinnerEntry {
                    id: ctx.telegram_user.id.0,
                    requests: 0,
                },
                &ctx.winnersdb_con,
            )
                .unwrap();

            // Retrieve the newly added winner
            Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
        })
        .unwrap();

    // If the user has requests left, send the question to the AI and return the AI's response
    if winner_entry.requests != 0 {
        let caller =
            if ctx.telegram_user.full_name().is_empty() || ctx.telegram_user.full_name() == "" {
                format!(
                    "({:?})",
                    ctx.telegram_user
                        .mention()
                        .is_none()
                        .then_some("(Anonymous)")
                )
            } else {
                let result = ctx
                    .telegram_user
                    .full_name()
                    .eq(&".".to_string())
                    .then_some("(Принтер)")
                    .unwrap_or(&format!("({})", &*ctx.telegram_user.full_name()))
                    .to_string();

                result
            };

        let formatted_question = format!("{} [{}] {}", caller,Local::now().format("%a-%H:%M"), ctx.command_content);

        let ai_respone = openai::helper_question(formatted_question)
            .await
            .map_err(|e| eprintln!("Error while interacting with openai api : {:?}", e));

        let _ = ctx
            .bot
            .send_message(ctx.msg.chat.id, ai_respone.unwrap())
            .reply_to_message_id(ctx.msg.id)
            .await;

        // Decrease the number of requests left for the user by 1
        update_winners_requests(
            &ctx.winnersdb_con,
            winner_entry.id,
            winner_entry.requests - 1,
        )
            .unwrap();

        return Ok(());
    }

    // If the user has no requests left, send a message to the user indicating this
    let _ = ctx
        .bot
        .send_message(
            ctx.msg.chat.id,
            "Извините, у вас нет запросов к gpt4.\n Команда /info для подробной информации.",
        )
        .reply_to_message_id(ctx.msg.id)
        .await;

    Ok(())
}