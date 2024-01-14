use config::CONFIG;
use log::*;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig, Client};
use lazy_static::lazy_static;
use std::collections::VecDeque;
use tokio::sync::Mutex;

lazy_static! {
    static ref MESSAGE_HISTORY: Mutex<VecDeque<ChatCompletionRequestMessage>> = Mutex::new(VecDeque::new());
}

// Установите подходящий размер для вашего приложения
const MAX_HISTORY_SIZE: usize = 10;

pub async fn helper_question(question: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new().with_api_key(CONFIG.clone().openai.openai_api_token);

    let chatgpt_prompt = &CONFIG.openai.helper_prompt_template;
    dbg!(chatgpt_prompt);

    let token_limit = CONFIG.openai.gpt_tokens_per_request_limit;

    let user_prompt = question;
    info!("Получен новый запрос от пользователя: {}", &user_prompt);

    let client = Client::with_config(config);

    let mut history = MESSAGE_HISTORY.lock().await;

    // Ограничиваем размер истории
    if history.len() > MAX_HISTORY_SIZE {
        history.pop_front(); // Удаляем самое старое сообщение
    }

    // Формирование запроса
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(token_limit as u16)
        .model("gpt-4")
        .messages({
            let mut msgs = history.iter().cloned().collect::<Vec<_>>();
            msgs.push(
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::System)
                    .content(chatgpt_prompt)
                    .build()?
            );
            msgs.push(
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(&user_prompt)
                    .build()?
            );
            msgs
        })
        .build()?;

    let response = client.chat().create(request).await?;

    // Добавление нового сообщения пользователя в историю
    history.push_back(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(user_prompt)
            .build()?
    );

    Ok(response.choices[0].message.content.clone().ok_or("No content in response")?)
}
