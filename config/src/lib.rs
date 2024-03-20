mod error;
use error::ConfigError;

use chrono::prelude::*;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};
use std::fs;
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
    pub character_tokens_per_request_limit: u32,
    pub gpt_tokens_per_request_limit: u32,
    pub default_prompt_template: String,
    pub helper_prompt_template: String,
    pub prompt_blacklist_words: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarEntry {
    pub date: String,
    pub character: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CalendarConfig {
    pub plan: Vec<CalendarEntry>,
}
impl CalendarConfig {
    pub fn try_get_daily_character_names(&self) -> Result<Vec<String>, ()> {
        let formatted_date = {
            let utc_now: DateTime<Utc> = Utc::now();
            let date = utc_now.date_naive();

            date.format("%y-%m-%d").to_string()
        };

        for entry in &self.plan {
            if entry.date == formatted_date {
                return Ok(entry.character.clone());
            }
        }

        Err(())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub openai: OpenAIConfig,
    pub calendar: CalendarConfig,
}

impl Config {
    fn load_from_current_path() -> Result<Config, ConfigError> {
        let mut config_path = std::env::current_dir()?;
        config_path.push("config.toml");

        let config_text = fs::read(config_path)?;
        let config_text = String::from_utf8(config_text)?;

        let config = from_str(&config_text)?;
        Ok(config)
    }
}

lazy_static! {
    pub static ref CONFIG: Config = Config::load_from_current_path().unwrap();
}
