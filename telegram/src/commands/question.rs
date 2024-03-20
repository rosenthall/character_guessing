use crate::handler::CommandContext;
use config::CONFIG;
use log::{info, trace};
use teloxide::payloads::SendMessageSetters;
use teloxide::requests::Requester;
use database::update_questions_quantity;


pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New question : {}", ctx.command_content);

    // Check if the question is too short
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

    // Check if the user has already asked more than three questions
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

    // Check if the question contains any blacklisted words
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

    // Send the question to the AI and get the response
    let mut ai_answer = openai::character_question(ctx.command_content).await;
    info!("AI ANSWER : {}", ai_answer.clone());

    // Check if the response contains any of the names of the characters for the day
    let names = CONFIG.calendar.try_get_daily_character_names().unwrap();
    if names
        .iter()
        .any(|name| ai_answer.to_lowercase().contains(&name.to_lowercase()))
    {
        // If a name is found, censor it
        for name in &names {
            ai_answer = ai_answer.replace(name, "[ИМЯ ПЕРСОНАЖА]");
        }
    }

    // Send the AI's response to the user
    let _ = ctx
        .bot
        .send_message(ctx.msg.chat.id, ai_answer)
        .reply_to_message_id(ctx.msg.id)
        .await;

    // Increase the number of questions asked by the user by 1
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