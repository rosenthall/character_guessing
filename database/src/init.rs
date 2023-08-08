use rusqlite::{params, Connection};

use std::result::Result;
use std::time::Duration;

use log::*;
use crate::control;
use crate::control::{update_db_connection};

// Функция для создания новой базы данных и таблицы
pub fn create_database_and_table(date: &str) -> Result<Connection, ()> {
    // Создаем подключение к базе данных

    let db_path = format!("data/{}.db", date);

    info!("Путь к файлу новой базы данных: {}", db_path.clone());

    let conn = Connection::open(db_path).unwrap_or_else(|e| {
        error!("Error while creating a connction : {e}");
        panic!();
    });

    info!("Создаю новую таблицу в базе!");
    // Создаем таблицу Users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Users (
                  ID INTEGER PRIMARY KEY NOT NULL,
                  attempts INTEGER CHECK (attempts >= 0 AND attempts <= 5),
                  is_won BOOLEAN,
                  questions_quantity INTEGER CHECK (questions_quantity >= 0 AND questions_quantity <= 3)
                  )",
        params![],
    ).unwrap_or_else(|e| {
        error!("Ошибка при создании базы данных : {e:#?}");
        panic!();
    });

    Ok(conn)
}


pub async fn update_db_each_day_service() {
    info!("DB updating Service started!");

    loop {

        tokio::spawn( async {
            //Ожидаем 24 часа перед исполнением кода

            let _ = tokio::time::sleep(Duration::from_secs(24*60*60)).await;
            let formatted_date = control::get_current_formatted_date();

            // Инициализируем новую базу данных
            let new_db = create_database_and_table(&formatted_date).unwrap();
            update_db_connection(new_db).await;


        }).await.unwrap();
    }
}