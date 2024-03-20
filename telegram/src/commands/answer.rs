use crate::CommandContext;
use config::CONFIG;
use std::{ops::Mul, result::Result};
use teloxide::{payloads::SendMessageSetters, prelude::Requester};

use database::{
    model::WinnerEntry,
    winners::{try_add_winner, try_get_winner, update_winners_requests},
};
use strsim::normalized_damerau_levenshtein;

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    // Delete the user's message
    ctx.bot.delete_message(ctx.msg.chat.id, ctx.msg.id).await.unwrap();

    // If the user has already won, send a message indicating this
    if ctx.db_entry_user.is_won {
        let _ = ctx
            .bot
            .send_message(
                ctx.msg.chat.id,
                &format!(
                    "{}, ты уже победил сегодня!",
                    ctx.telegram_user.mention().unwrap_or(ctx.telegram_user.first_name)
                ),
            )
            .await;
        return Ok(());
    }

    // If the user has exceeded their guess attempts, send a message indicating this
    if ctx.db_entry_user.attempts >= 5 {
        let _ = ctx
            .bot
            .send_message(
                ctx.msg.chat.id,
                "Извини, но ты уже сделал 5 попыток отгадать персонажа сегодня!Возвращайся завтра \
                 и попробуй угадать следуйщего персонажа.",
            )
            .reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

    // Get the list of the character's names for the day
    let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();

    // Convert the names to lowercase
    let character_names = character_names.iter().map(|i| i.to_lowercase()).collect::<Vec<String>>();

    for name in character_names {
        // Compare the user's guess to the character's names
        // Multiply the result by 100 for easier comparison
        let res: u8 = normalized_damerau_levenshtein(&ctx.command_content, &name.to_lowercase())
            .mul(100.0) as u8;

        // If the similarity is between 60 and 100, the user wins
        return match res {
            60..=100 => {
                // Debug print the result
                dbg!(res.clone());

                // Send a message indicating that the user has guessed the character
                let _ = ctx
                    .bot
                    .send_message(
                        ctx.msg.chat.id,
                        format!(
                            "Пользователь {} отгадал сегодняшнего персонажа!",
                            ctx.telegram_user
                                .mention()
                                .unwrap_or(ctx.telegram_user.clone().first_name)
                        ),
                    )
                    .await;

                // Update the user's win status in the database
                let _ = database::update_is_won(&ctx.con, ctx.telegram_user.id.0, true);

                // Try to get the user from the winners database
                let winner_entry =
                    try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con)
                        .or_else(|| {
                            // If the user does not exist in the winners database, add them
                            try_add_winner(
                                WinnerEntry { id: ctx.telegram_user.id.0, requests: 0 },
                                &ctx.winnersdb_con,
                            )
                            .unwrap();

                            // Get the newly added user from the winners database
                            Some(
                                try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con)
                                    .unwrap(),
                            )
                        })
                        .unwrap();

                // Add 3 requests to the user's total in the winners database
                let _ = update_winners_requests(
                    &ctx.winnersdb_con,
                    ctx.telegram_user.id.0,
                    winner_entry.requests + 3,
                );

                Ok(())
            }
            // If the similarity is less than 60, the user's guess is incorrect
            _ => {
                // Send a message indicating that the user's guess was incorrect
                let _ = ctx
                    .bot
                    .send_message(
                        ctx.msg.chat.id,
                        &format!(
                            "{}, вам не удалось угадать персонажа!",
                            ctx.telegram_user
                                .mention()
                                .unwrap_or(ctx.telegram_user.first_name.clone())
                        ),
                    )
                    .await;
                Ok(())
            }
        };
    }

    Ok(())
}
