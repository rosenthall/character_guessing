use crate::CommandContext;
use std::result::Result;
use teloxide::prelude::Requester;
use teloxide::payloads::SendMessageSetters;
use config::CONFIG;
use database::{check_user, update_is_won};

pub async fn execute(ctx : CommandContext<'_>) -> Result<(), ()> {

    let user = check_user(ctx.telegram_user.id.0, &ctx.con).unwrap();


    ctx.bot.delete_message(ctx.msg.chat.id, ctx.msg.id).await.unwrap();

    if user.is_won {
      let _ = ctx.bot.send_message(ctx.msg.chat.id, "Ты уже победил сегодня!")
        .reply_to_message_id(ctx.msg.id)
      .await;
    }

    //Если человек попытался угадать больше 5 раз - отказываем.
    if user.attempts >= 5 {
        ctx.bot.send_message(
            ctx.msg.chat.id,
                    "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
                ).reply_to_message_id(ctx.msg.id)
                    .await
                    .unwrap();
                return Ok(());
            }

            let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();

            if character_names.contains(&ctx.command_content) {
                let _ = ctx.bot.send_message(
                    ctx.msg.chat.id,
                    format!(
                        "Пользователь {} отгадал сегодняшнего персонажа!",
                        ctx.telegram_user.mention().unwrap_or(ctx.telegram_user.clone().first_name)))
                    .await;

                update_is_won(&ctx.con, ctx.db_entry_user.id, true).unwrap()
            } else {
                let _ = ctx.bot.send_message(ctx.msg.chat.id, &format!("{}, вам не удалось угадать персонажа!", ctx.telegram_user.mention().unwrap_or(ctx.telegram_user.first_name.clone())))
                    .await;
            };

    Ok(())
}