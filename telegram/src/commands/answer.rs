use crate::CommandContext;
use config::CONFIG;
use database::{check_user, update_is_won};
use std::ops::Mul;
use std::result::Result;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::Requester;

use strsim::normalized_damerau_levenshtein;

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    let user = check_user(ctx.telegram_user.id.0, &ctx.con).unwrap();

    ctx.bot
        .delete_message(ctx.msg.chat.id, ctx.msg.id)
        .await
        .unwrap();

    if user.is_won {
        let _ = ctx
            .bot
            .send_message(ctx.msg.chat.id, "Ты уже победил сегодня!")
            .reply_to_message_id(ctx.msg.id)
            .await;
        return Ok(());
    }

    //Если человек попытался угадать больше 5 раз - отказываем.
    if user.attempts >= 5 {
        ctx.bot
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

                update_is_won(&ctx.con, ctx.db_entry_user.id, true).unwrap();

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
