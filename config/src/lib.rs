use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;

#[derive(Debug, Serialize, Deserialize)]
struct TelegramConfig {
    telegram_token: String,
    telegram_allowed_groups: Vec<String>,
    telegram_admin_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OpenAIConfig {
    openai_api_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalendarEntry {
    date: String,
    prompt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CalendarConfig {
    plan: Vec<CalendarEntry>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    telegram: TelegramConfig,
    openai: OpenAIConfig,
    calendar: CalendarConfig,
}

impl Config {
    fn try_get_from_current_path() -> Result<Config, Err(e)> {
        let dir = std::env::current_dir()
            .expect("Cannot get current dir!");

        let config_path = format!("{}/config.toml", dir.to_str().unwrap());


        let config_text = std::fs::read(config_path)
            .expect("Cannot read file in current path!");

        let config_text = String::from_utf8(config_text).unwrap();

        let result = match toml::from_str(config_text) {
            Ok(toml) => {
                toml
            }
            Err(error) => {
                panic!("{}", error);
            }
            _ => {}
        };

    }
}