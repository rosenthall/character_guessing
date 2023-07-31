use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde::de::Error;
use std::string::FromUtf8Error;
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

#[derive(Debug)]
enum ConfigError {
    IoError(std::io::Error),
    TomlError(toml::de::Error),
    Utf8Error(FromUtf8Error),
}
impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError::IoError(error)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(error: toml::de::Error) -> Self {
        ConfigError::TomlError(error)
    }
}

impl From<FromUtf8Error> for ConfigError {
    fn from(error: FromUtf8Error) -> Self {
        ConfigError::Utf8Error(error)
    }
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