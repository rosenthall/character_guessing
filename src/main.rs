use log::info;
use pretty_env_logger::env_logger;

use chrono::prelude::*;
use config::CONFIG;

use database::init::update_db_each_day_service;

// Main function of the program
#[tokio::main]
async fn main() {
    // Initialize the logger
    env_logger::init();

    // Log that the program has started
    info!("Program has started!");

    // Get the current date and format it
    let formatted_date = {
        let utc_now: DateTime<Utc> = Utc::now();
        let date = utc_now.date_naive();

        date.format("%y-%m-%d").to_string()
    };

    // Log the current date
    info!("Current date : {formatted_date}");

    // Create a new thread to handle the database update service
    let db_updating_handle = tokio::spawn(async move {
        update_db_each_day_service().await;
    });

    // Create a new thread to handle the telegram bot
    let telegram_handle = tokio::spawn(async move {
        telegram::start_bot(&CONFIG.telegram).await;
    });

    // Wait for the telegram bot thread to finish
    telegram_handle.await.unwrap();
    // Wait for the database update service thread to finish
    db_updating_handle.await.unwrap();
}
