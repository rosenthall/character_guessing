// Importing necessary modules and packages
use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use config::CONFIG;
use log::*;

// Function to handle a character question
// This function sends a request to the OpenAI API and returns the response
pub async fn character_question(question: String) -> String {
    // Initialize the OpenAI configuration
    let config = OpenAIConfig::new().with_api_key(CONFIG.clone().openai.openai_api_token);

    // Get the character names for the day
    let character_names = CONFIG.calendar.try_get_daily_character_names().unwrap();

    // Format the prompt for the chatbot
    let chatgpt_prompt =
        format!("{} {}", CONFIG.clone().openai.default_prompt_template, character_names[0]);

    // Log the chatbot prompt
    dbg!(chatgpt_prompt.clone());

    // Get the maximum number of tokens for the request
    let token_limit = CONFIG.openai.clone().character_tokens_per_request_limit;

    // Get the user's prompt
    let user_prompt = question;
    info!("Received a new request from the user: {}", &user_prompt.clone());

    // Create a new client with the OpenAI configuration
    let client = Client::with_config(config);

    // Form the request
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(token_limit as u16)
        .model("gpt-4")
        .messages([
            // Message with the "System" role to set the assistant's context
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(
                    "You are a historical character.You're trying to hide your name. You are \
                     responding in the language in which you're being asked..",
                )
                .build()
                .unwrap(),
            // Message with the "ChatGPT" role and the chatbot's prompt
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant) // Here the assistant plays the role of ChatGPT
                .content(chatgpt_prompt)
                .build()
                .unwrap(),
            // Message with the "User" role and the user's question
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&user_prompt)
                .build()
                .unwrap(),
        ])
        .build()
        .unwrap();

    // Send the request and get the response
    let response = client.chat().create(request).await.unwrap();
    info!("Response from openai api: {:#?}", response);

    // Return the content of the response
    response.choices[0].message.content.clone().unwrap()
}
