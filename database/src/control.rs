use crate::init::create_database_and_table;
use chrono::prelude::*;
use lazy_static::lazy_static;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

lazy_static! {
    pub static ref DATABASE: Arc<Mutex<Connection>> = {
        let formatted_date = {
            let utc_now: DateTime<Utc> = Utc::now();
            let date = utc_now.date_naive();

            date.format("%y-%m-%d").to_string()
        };

        let con = create_database_and_table(&formatted_date).unwrap();

        Arc::new(Mutex::new(con))
    };
};

impl DATABASE {
    // Функция для получения соединения (async)
    pub async fn try_get_connection(&self) -> MutexGuard<'_, Connection> {
        // Блокируем мьютекс, чтобы получить доступ к Connection
        let connection = self.lock().await;
        connection // Возвращаем клон Connection
    }
}
