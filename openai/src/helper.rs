// Importing necessary modules and packages
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

// Initialize static variables for initial and recent messages
lazy_static! {
    static ref INIT_MESSAGES: Mutex<Vec<ChatCompletionRequestMessage>> = Mutex::new(Vec::new());
    static ref RECENT_MESSAGES: Mutex<VecDeque<ChatCompletionRequestMessage>> =
        Mutex::new(VecDeque::new());
}

// Maximum number of recent messages to keep
const MAX_RECENT_MESSAGES: usize = 6;

// Function to add an initial message to the list
pub async fn add_init_message(msg: ChatCompletionRequestMessage) {
    let mut init_messages = INIT_MESSAGES.lock().await;
    init_messages.push(msg);
}

// Function to handle a helper question
// This function sends a request to the OpenAI API and returns the response
pub async fn helper_question(question: String) -> Result<String, Box<dyn std::error::Error>> {
    // Initialize the OpenAI configuration
    let config = OpenAIConfig::new().with_api_key(CONFIG.clone().openai.openai_api_token);

    // Get the prompt for the chatbot
    let chatgpt_prompt = &CONFIG.openai.helper_prompt_template;

    // Add initial messages to the list
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
            .role(Role::Assistant) // Here the assistant plays the role of ChatGPT
            .content(chatgpt_prompt)
            .build()
            .unwrap(),
    )
    .await;

    // Get the token limit for the request
    let token_limit = CONFIG.openai.gpt_tokens_per_request_limit;

    // Get the user's prompt
    let user_prompt = question;
    info!("Received a new request from the user: {}", &user_prompt);

    // Create a new client with the OpenAI configuration
    let client = Client::with_config(config);

    // Lock the initial and recent messages
    let init_messages = INIT_MESSAGES.lock().await;
    let mut recent_history = RECENT_MESSAGES.lock().await;

    // Limit the size of the recent messages history
    if recent_history.len() > MAX_RECENT_MESSAGES {
        recent_history.pop_front();
    }

    // Form the request
    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(token_limit as u16)
        .model("gpt-4")
        .messages({
            let mut msgs = init_messages.iter().cloned().collect::<Vec<_>>(); // Add initialization messages
            msgs.extend(recent_history.iter().cloned()); // Add the last 10 messages
            msgs.push(
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(&user_prompt)
                    .build()?,
            );
            msgs
        })
        .build()?;

    // Send the request and get the response
    let response = client.chat().create(request).await?;

    info!("Response from openai : {:#?}", response.clone());
    // Add the new user message to the history of regular messages
    recent_history.push_back(
        ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(user_prompt)
            .build()?,
    );

    // Return the content of the response
    Ok(response.choices[0]
        .message
        .content
        .clone()
        .ok_or("No content in response")?)
}