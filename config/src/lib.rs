mod error;
use error::ConfigError;

use serde_derive::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use lazy_static::lazy_static;
use toml::from_str;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TelegramConfig {
    pub telegram_token: String,
    pub telegram_allowed_groups: Vec<String>,
    pub telegram_admin_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIConfig {
    pub openai_api_token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarEntry {
    pub date: String,
    pub prompt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarConfig {
    pub plan: Vec<CalendarEntry>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub openai: OpenAIConfig,
    pub calendar: CalendarConfig,
}
impl Config {
    fn load_from_current_path() -> Result<Config, ConfigError> {
        let dir = std::env::current_dir()?;
        let mut config_path = PathBuf::from(dir);
        config_path.push("config.toml");

        let config_text = fs::read(config_path)?;
        let config_text = String::from_utf8(config_text)?;

        let config = from_str(&config_text)?;
        Ok(config)
    }
}

lazy_static! {
    pub static ref CONFIG : Config = Config::load_from_current_path().unwrap();
}
