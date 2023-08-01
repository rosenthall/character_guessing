use std::fs;
use rusqlite::{params, Connection, Result};
use log::*;
// Функция для создания новой базы данных и таблицы
pub fn create_database_and_table(date: &str) -> Result<()> {
    // Создаем подключение к базе данных

    let db_path = format!("databases/{}.db", date);

    info!("Путь к файлу новой базы данных: {}", db_path.clone());


    let conn = Connection::open(db_path)?;

    info!("Создаю новую таблицу в базе!");
    // Создаем таблицу Users
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Users (
                  ID INTEGER PRIMARY KEY NOT NULL,
                  attempts INTEGER CHECK (attempts >= 1 AND attempts <= 5),
                  is_won BOOLEAN,
                  questions_quantity INTEGER CHECK (questions_quantity >= 1 AND questions_quantity <= 3)
                  )",
        params![],
    ).unwrap_or_else(|e| {
        error!("Ошибка при создании базы данных : {e:#?}");
        panic!();
        0
    });

    Ok(())
}

