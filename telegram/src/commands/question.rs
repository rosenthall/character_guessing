use log::{info, trace};
use teloxide::requests::Requester;
use crate::command::{CommandContext};
use teloxide::payloads::SendMessageSetters;

use database::update_questions_quantity;

pub async fn execute(ctx : CommandContext<'_>) -> Result<(), ()> {
    info!("New question : {}", ctx.command_content);

    // Минимальная длинна вопроса. Если вопрос короче 5 символов, то и отвечать смысла нет.
    if ctx.command_content.len() <= 5  {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            "После команды \"/question\" должен следовать вопрос. \n Подробнее - /info",
        ).reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

//Если человек уже задал больше трех вопросов - отказываем.
    if ctx.db_entry_user.questions_quantity >= 3 {
        ctx.bot.send_message(
            ctx.msg.chat.id,
            "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
        ).reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

    let ai_answer = openai::character_question(ctx.command_content).await;
    info!("AI ANSWER : {}", ai_answer.clone());

    let _ = ctx.bot.send_message(ctx.msg.chat.id, ai_answer).reply_to_message_id(ctx.msg.id).await;

    //Увеличиваем количество заданных вопросов на 1
    trace!(
                "Увеличенно количество заданных вопросов на 1 для пользователя : {}",
                ctx.telegram_user.id
            );
    update_questions_quantity(&ctx.con, ctx.db_entry_user.id, ctx.db_entry_user.questions_quantity + 1).unwrap();
    Ok(())
}

