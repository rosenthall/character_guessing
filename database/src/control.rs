use crate::init::create_database_and_table;
use async_trait::async_trait;
use chrono::prelude::*;
use log::info;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub(crate) fn get_current_formatted_date() -> String {
    let formatted_date = {
        let utc_now: DateTime<Utc> = Utc::now();
        let date = utc_now.date_naive();

        date.format("%y-%m-%d").to_string()
    };

    formatted_date
}

#[async_trait]
trait ControlDatabase {
    //Просто конструктор
    fn from(c: Arc<Mutex<Connection>>) -> Self;

    //Функция для самой первой инициализации базы.
    fn new_() -> Self;

    //Функция возвращает подключение к базе данных
    async fn get_connection(&self) -> MutexGuard<Connection>;

    //Функция обновляет текущее подключение к базе, в частности ставит новую дату и создает новый файл, если дата изменилась.
    async fn update(&mut self);
}

pub type DatabaseHandler = Arc<Mutex<Connection>>;

#[async_trait]
impl ControlDatabase for DatabaseHandler {
    fn from(c: Arc<Mutex<Connection>>) -> Self {
        c
    }

    fn new_() -> Self {
        let formatted_date = get_current_formatted_date();

        let con = create_database_and_table(&formatted_date).unwrap();

        Arc::new(Mutex::new(con))
    }

    async fn get_connection(&self) -> MutexGuard<Connection> {
        let connection = self.lock().await;

        connection
    }

    async fn update(&mut self) {
        let formatted_date = get_current_formatted_date();

        let con = create_database_and_table(&formatted_date).unwrap();
        self = &mut ControlDatabase::from(Arc::new(Mutex::new(con)));
    }
}

pub static DATABASE_HANDLER: Lazy<DatabaseHandler> = Lazy::new(|| {
    let handle = DatabaseHandler::new_();

    handle
});

pub(crate) async fn update_db_connection(con: Connection) {
    *DATABASE_HANDLER.lock().await = con;
    info!("Updated db connection!");
}
