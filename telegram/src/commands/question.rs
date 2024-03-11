use crate::command::CommandContext;
use config::CONFIG;
use log::{info, trace};
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;

use database::update_questions_quantity;

//noinspection ALL

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New question : {}", ctx.command_content);

    // Минимальная длинна вопроса. Если вопрос короче 5 символов, то и отвечать смысла нет.
    if ctx.command_content.len() <= 5 {
        ctx.bot
            .send_message(
                ctx.msg.chat.id,
                "После команды \"/question\" должен следовать вопрос. \n Подробнее - /info",
            )
            .reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

    //Если человек уже задал больше трех вопросов - отказываем.
    if ctx.db_entry_user.questions_quantity >= 3 {
        ctx.bot
            .send_message(
                ctx.msg.chat.id,
                "Извини, но ты уже задал свои 3 вопроса сегодня!\
                Возвращайся завтра и попробуй угадать следуйщего персонажа.",
            )
            .reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();
        return Ok(());
    }

    // Если в вопросе есть запрещенные слова - отказываем.
    if CONFIG
        .openai
        .prompt_blacklist_words
        .iter()
        .any(|f| ctx.command_content.clone().to_lowercase().contains(f))
    {
        ctx.bot
            .send_message(
                ctx.msg.chat.id,
                "Извини, в твоём сообщении есть запрещенные слова!\nПопробуй перефразировать или не задавать прямых вопросов касающихся имени персонажа!",
            )
            .reply_to_message_id(ctx.msg.id)
            .await
            .unwrap();

        return Ok(());
    }

    let mut ai_answer = openai::character_question(ctx.command_content).await;
    info!("AI ANSWER : {}", ai_answer.clone());

    // Проверяем сообщение на наличие одного из сегодняшних имен. Если оно есть - цензурим.
    let names = CONFIG.calendar.try_get_daily_character_names().unwrap();
    if names
        .iter()
        .any(|name| ai_answer.to_lowercase().contains(&name.to_lowercase()))
    {
        for name in &names {
            ai_answer = ai_answer.replace(name, "[ИМЯ ПЕРСОНАЖА]");
        }
    }

    let _ = ctx
        .bot
        .send_message(ctx.msg.chat.id, ai_answer)
        .reply_to_message_id(ctx.msg.id)
        .await;

    //Увеличиваем количество заданных вопросов на 1
    trace!(
        "Увеличенно количество заданных вопросов на 1 для пользователя : {}",
        ctx.telegram_user.id
    );
    update_questions_quantity(
        &ctx.con,
        ctx.db_entry_user.id,
        ctx.db_entry_user.questions_quantity + 1,
    )
    .unwrap();
    Ok(())
}
