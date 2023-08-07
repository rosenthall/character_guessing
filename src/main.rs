use log::info;
use pretty_env_logger::env_logger;

use chrono::prelude::*;
use config::CONFIG;

#[tokio::main]
async fn main() {
    env_logger::init();

    info!("Program has started!");

    let formatted_date = {
        let utc_now: DateTime<Utc> = Utc::now();
        let date = utc_now.date_naive();

        date.format("%y-%m-%d").to_string()
    };

    info!("Current date : {formatted_date}");

    //Создаем базу данных с названием на основе сегодняшней даты
    //let db_connection = database::init::create_database_and_table(formatted_date.as_str()).unwrap();

    //Запускаем телеграм бота и передаем ему TelegramConfig.
    telegram::start_bot(&CONFIG.telegram).await;
}
