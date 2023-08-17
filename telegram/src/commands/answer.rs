use crate::CommandContext;
use config::CONFIG;
use std::ops::Mul;
use std::result::Result;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;

use strsim::normalized_damerau_levenshtein;
use database::model::WinnerEntry;
use database::winners::{try_add_winner, try_get_winner, update_winners_requests};

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {

    ctx.bot
        .delete_message(ctx.msg.chat.id, ctx.msg.id)
        .await
        .unwrap();

    if ctx.db_entry_user.is_won {
        let _ = ctx
            .bot
            .send_message(ctx.msg.chat.id, &format!("{}, ты уже победил сегодня!", ctx.telegram_user.mention().unwrap_or(ctx.telegram_user.first_name)))
            .await;
        return Ok(());
    }

    //Если человек попытался угадать больше 5 раз - отказываем.
    if ctx.db_entry_user.attempts >= 5 {
        let _ = ctx.bot
            .send_message(
                ctx.msg.chat.id,
                "Извини, но ты уже сделал 5 попыток отгадать персонажа сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
            )
            .reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

    // Получаем список имен сегодняшнего персонажа
    let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();

    // Приводим в нижний регистр
    let character_names = character_names
        .iter()
        .map(|i| i.to_lowercase())
        .collect::<Vec<String>>();

    for name in character_names {
        // Сравниваем наши имена в нижнем регистре и ответ пользователя, после чего умножаем результат на 100 для более удобного сравнения

        let res: u8 = normalized_damerau_levenshtein(&ctx.command_content, &name.to_lowercase())
            .mul(100.0) as u8;
        return match res {
            // От 60 до 100 схожести означает победу.
            60..=100 => {
                dbg!(res.clone());

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
                // Обновляем показания в сегодняшней тоже.
                let _ = database::update_is_won(&ctx.con, ctx.telegram_user.id.0, true);



                // Ищем пользователя в постоянной базе данных. Если его там нет - добавляем.
                let winner_entry  = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).or_else(|| {

                    // Добавляем если такого поля нет.
                    try_add_winner(WinnerEntry {
                        id: ctx.telegram_user.id.0,
                        requests: 0
                    }, &ctx.winnersdb_con).unwrap();

                    Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
                }).unwrap();


                // Добавляем ему +3 запроса (сообственно награда)
                let _ = update_winners_requests(&ctx.winnersdb_con, ctx.telegram_user.id.0, winner_entry.requests+3);


                Ok(())
            }
            _ => {
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
        }
    }

    Ok(())
}
