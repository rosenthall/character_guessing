use crate::command::CommandContext;
use log::{info, trace};
use teloxide::payloads::{SendMessageSetters};
use teloxide::requests::Requester;
use database::model::WinnerEntry;
use database::winners::*;

use database::{try_add_user, update_questions_quantity};


pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New gpt question : {}", ctx.command_content);


    let _ = ctx
        .bot
        .send_message(
            ctx.msg.chat.id,
            "Вот ссылка на мануал : https://telegra.ph/Kak-polzovatsya-botom-08-09",
        )
        .reply_to_message_id(ctx.msg.id)
        .await;


    // получаем доступ к записи о пользователе который вызвал комманду. Если записи нет - добавляем его туда.
    let winner_entry  = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).or_else(|| {

        // Добавляем если такого поля нет.
        try_add_winner(WinnerEntry {
            id: ctx.telegram_user.id.0,
            requests: 0
        }, &ctx.winnersdb_con).unwrap();

        Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
    }).unwrap();

    // Если у пользователя есть запросы
    if winner_entry.id <= 0 {
        return Ok(())
    }

    //В случае если оставшихся запросов нет - отправляем сообщение об этом.
    let _ = ctx.bot
        .send_message(ctx.msg.chat.id, "Извините, у вас нет запросов к gpt4. \n Команда /info для подробной информации.")
        .reply_to_message_id(ctx.msg.id)
        .await;


    Ok(())
}

