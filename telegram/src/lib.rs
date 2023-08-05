mod commands;

use commands::*;
use log::info;

use teloxide::prelude::*;

use config::TelegramConfig;

pub async fn start_bot(cfg: &TelegramConfig) {
    let bot = Bot::new(&cfg.telegram_token);

    info!("Bot started as : {:?}", bot.get_me().await.unwrap());

    let _ = Command::repl(bot, handle_command).await;
}
