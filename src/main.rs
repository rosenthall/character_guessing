use log::info;
use pretty_env_logger::env_logger;

use chrono::prelude::*;
use config::CONFIG;

use database::init::update_db_each_day_service;

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

    //Handle сервиса для обновления баз данных в отдельном треде
    let db_updating_handle = tokio::spawn(async move {
        update_db_each_day_service().await;
    });

    //handle для запуска телеграм бота.
    let telegram_handle = tokio::spawn(async move {
        telegram::start_bot(&CONFIG.telegram).await;
    });

    telegram_handle.await.unwrap();
    db_updating_handle.await.unwrap();
}
