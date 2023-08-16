use crate::command::CommandContext;
use log::{info, trace};
use teloxide::payloads::{SendMessageSetters};
use teloxide::requests::Requester;
use database::model::WinnerEntry;
use database::winners::*;

use database::{try_add_user, update_questions_quantity};


pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New gpt question : {}", ctx.command_content);

    // получаем доступ к записи о пользователе который вызвал комманду. Если записи нет - добавляем его туда.
    let winner_entry  = try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).or_else(|| {

        // Добавляем если такого поля нет.
        try_add_winner(WinnerEntry {
            id: ctx.telegram_user.id.0,
            requests: 0
        }, &ctx.winnersdb_con).unwrap();

        Some(try_get_winner(ctx.telegram_user.clone().id.0, &ctx.winnersdb_con).unwrap())
    }).unwrap();

    // Если у пользователя есть запросы - сообственно делаем запрос к openai.
    if winner_entry.id <= 0 {

        let ai_respone = openai::helper_question(ctx.command_content).await;

        let _ = ctx.bot
            .send_message(ctx.msg.chat.id, ai_respone)
            .reply_to_message_id(ctx.msg.id)
            .await;

        //Отнимаем у этого пользователя 1 запрос.
        update_winners_requests(&ctx.winnersdb_con, winner_entry.id,winner_entry.requests-1).unwrap();

        return Ok(())
    }

    //В случае если оставшихся запросов нет - отправляем сообщение об этом.
    let _ = ctx.bot
        .send_message(ctx.msg.chat.id, "Извините, у вас нет запросов к gpt4. \n Команда /info для подробной информации.")
        .reply_to_message_id(ctx.msg.id)
        .await;


    Ok(())
}

