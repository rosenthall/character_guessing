use config::CONFIG;

use log::*;

use async_openai::types::{
    ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role,
};
use async_openai::{config::OpenAIConfig, Client};

pub async fn character_question(question: String) -> String {
    let config = OpenAIConfig::new().with_api_key(CONFIG.clone().openai.openai_api_token);

    // Получаем промпт для роли "ChatGPT" из конфигурации

    let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();
    let chatgpt_prompt = format!(
        "{} {}",
        CONFIG.clone().openai.default_prompt_template,
        character_names[0]
    );
    dbg!(chatgpt_prompt.clone());
    // Получаем максимальное количество токенов на один запрос к openai
    let token_limit = CONFIG.openai.clone().character_tokens_per_request_limit;

    // Получаем промпт для вопроса от пользователя (аргумент функции)
    let user_prompt = question;
    info!(
        "Получен новый запрос от пользователя : {}",
        &user_prompt.clone()
    );

    // Создаем клиента для работы с OpenAI API
    let client = Client::with_config(config);

    // Формируем запрос на создание чат-подобной модели с указанными ролями и сообщениями
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(token_limit as u16)
        .model("gpt-3.5-turbo")
        .messages([
            // Сообщение с ролью "System" для установки контекста ассистента
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a historical character.You're trying to hide your name. You are responding in the language in which you're being asked..")
                .build().unwrap(),

            // Сообщение с ролью "ChatGPT" и содержанием промпта для ChatGPT
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant) // Здесь ассистент играет роль ChatGPT
                .content(chatgpt_prompt)
                .build().unwrap(),

            // Сообщение с ролью "User" и содержанием вопроса пользователя
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&user_prompt)
                .build().unwrap(),
        ])
        .build().unwrap();

    let response = client.chat().create(request).await.unwrap();
    info!("Ответ от openai api : {:#?}", response);

    response.choices[0].message.content.clone().unwrap()
}
