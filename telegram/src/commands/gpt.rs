use crate::command::CommandContext;
use log::{info, trace};
use teloxide::payloads::{SendMessageSetters};
use teloxide::requests::Requester;
use config::CONFIG;
use database::winners::*;

use database::update_questions_quantity;

//noinspection ALL

pub async fn execute(ctx: CommandContext<'_>) -> Result<(), ()> {
    info!("New gpt_question : {}", ctx.command_content);

    // Создаем отдельное подключение к WINNERS_DB, так как ctx.con ведёт к сегодняшней дб, а не к постоянной.
    let con = WINNERS_DB.lock().unwrap();


    let ai_answer = openai::helper_question(ctx.command_content).await;
    info!("AI ANSWER : {}", ai_answer.clone());

    let _ = ctx
        .bot
        .send_message(ctx.msg.chat.id, ai_answer)
        .reply_to_message_id(ctx.msg.id)
        .await;

    update_questions_quantity(
        &ctx.con,
        ctx.db_entry_user.id,
        ctx.db_entry_user.questions_quantity + 1,
    )
        .unwrap();
    Ok(())
}
