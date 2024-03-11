use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
    CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig, Client};
use config::CONFIG;
use lazy_static::lazy_static;
use log::*;
use std::collections::VecDeque;
use tokio::sync::Mutex;

lazy_static! {
    static ref INIT_MESSAGES: Mutex<Vec<ChatCompletionRequestMessage>> = Mutex::new(Vec::new());
    static ref RECENT_MESSAGES: Mutex<VecDeque<ChatCompletionRequestMessage>> =
        Mutex::new(VecDeque::new());
}

const MAX_RECENT_MESSAGES: usize = 6;

pub async fn add_init_message(msg: ChatCompletionRequestMessage) {
    let mut init_messages = INIT_MESSAGES.lock().await;
    init_messages.push(msg);
}

pub async fn helper_question(question: String) -> Result<String, Box<dyn std::error::Error>> {
    let config = OpenAIConfig::new().with_api_key(CONFIG.clone().openai.openai_api_token);

    let chatgpt_prompt = &CONFIG.openai.helper_prompt_template;

    add_init_message(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::System)
            .content(
                "You're a helpful bot in our chat room called \"Nyagpt\", you are catgirl.",
            )
            .build()
            .unwrap(),
    )
    .await;

    add_init_message(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::Assistant) // Здесь ассистент играет роль ChatGPT
            .content(chatgpt_prompt)
            .build()
            .unwrap(),
    )
    .await;

    let token_limit = CONFIG.openai.gpt_tokens_per_request_limit;

    let user_prompt = question;
    info!("Получен новый запрос от пользователя: {}", &user_prompt);

    let client = Client::with_config(config);

    let init_messages = INIT_MESSAGES.lock().await;
    let mut recent_history = RECENT_MESSAGES.lock().await;

    // Ограничиваем размер истории обычных сообщений
    if recent_history.len() > MAX_RECENT_MESSAGES {
        recent_history.pop_front();
    }

    // Формирование запроса
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(token_limit as u16)
        .model("gpt-4")
        .messages({
            let mut msgs = init_messages.iter().cloned().collect::<Vec<_>>(); // Добавление сообщений инициализации
            msgs.extend(recent_history.iter().cloned()); // Добавление последних 10 сообщений
            msgs.push(
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(&user_prompt)
                    .build()?,
            );
            msgs
        })
        .build()?;

    let response = client.chat().create(request).await?;

    info!("Response from openai : {:#?}", response.clone());
    // Добавление нового сообщения пользователя в историю обычных сообщений
    recent_history.push_back(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(user_prompt)
            .build()?,
    );

    Ok(response.choices[0]
        .message
        .content
        .clone()
        .ok_or("No content in response")?)
}
