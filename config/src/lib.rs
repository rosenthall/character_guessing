mod error;
use error::ConfigError;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use toml::from_str;


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
    pub fn load_from_current_path() -> Result<Config, ConfigError> {
        let dir = std::env::current_dir()?;
        let mut config_path = PathBuf::from(dir);
        config_path.push("config.toml");

        let config_text = fs::read(config_path)?;
        let config_text = String::from_utf8(config_text)?;

        let config = from_str(&config_text)?;
        Ok(config)
    }
}