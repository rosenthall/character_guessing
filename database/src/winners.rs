// В проекте два типа баз данных:
// 1. Временные, те которые создаются каждый день.
// 2. Постоянная база которая хранит в себе количество запросов к chatgpt4 у пользователей.


use std::error::Error;
use rusqlite::{Connection, params};
use std::sync::{Arc};
use tokio::sync::{MutexGuard,Mutex};
use log::{error, trace};
use once_cell::sync::Lazy;
use crate::model::WinnerEntry;

pub type WinnersDbPool = Arc<Mutex<Connection>>;

pub static WINNERS_DB: Lazy<WinnersDbPool> = Lazy::new(|| {
    let connection = Connection::open("data/winners.db").expect("Failed to open database");

    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS Winners (
                id INTEGER PRIMARY KEY,
                requests INTEGER
            )",
            [],
        )
        .expect("Failed to create table");

    Arc::new(Mutex::new(connection))
});


pub fn try_add_winner(user: WinnerEntry, con: &Connection) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO Winners (id, requests) VALUES (?, ?)";

    // Выполняем запрос с помощью метода execute, передавая параметры
    con.execute(
        query,
        params![
            user.id as u64, // Предполагаем, что тип поля ID в базе данных - INTEGER
            user.requests,
        ],
    )
        .unwrap_or_else(|e| {
            error!("Ошибка во время подготовки query : {e}");
            panic!();
        });

    Ok(())
}

pub fn try_get_winner(id : u64, con: &Connection) -> Option<WinnerEntry> {
    let query = "SELECT id, requests FROM Winners WHERE ID = ?";

    let mut stmt = con.prepare(query).unwrap_or_else(|e| {
        error!("Ошибка во время подготовки query : {e}");
        panic!();
    });

    let result = stmt.query_row(params![id], |row| {
        Ok(WinnerEntry {
            id: row.get(0).expect("Failed to get id"),
            requests: row.get(1).expect("Failed to get attempts")
        })
    });

    match result {
        Ok(user) => Some(user),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(err) => {
            eprintln!("Error fetching user: {:?}", err);
            None
        }
    }
}


pub fn update_winners_requests(
    connection: &Connection,
    user_id: u64,
    attempts: u16,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Изменяю количество запросов для пользователя : {}, новое значение : {}",
        user_id.clone(),
        attempts.clone()
    );

    connection
        .execute(
            "UPDATE Winners SET requests = ?1 WHERE id = ?2",
            [attempts as u16, user_id.try_into().unwrap()],
        )
        .unwrap();
    Ok(())
}
