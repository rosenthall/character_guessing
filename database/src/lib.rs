pub mod control;
mod init;
pub mod model;

use model::User;

use std::error::Error;
use std::result::Result;

use log::{error, trace};
use rusqlite::{params, Connection};

// Если пользователь есть в актуальной дб - возвращает структуру User, в противном случае возвращает None
pub fn check_user(id: u64, con: &Connection) -> Option<model::User> {
    let query = "SELECT ID, attempts, is_won, questions_quantity FROM Users WHERE ID = ?";
    let id = id as i64;

    let mut stmt = con.prepare(query).unwrap_or_else(|e| {
        error!("Ошибка во время подготовки query : {e}");
        panic!();
    });

    let result = stmt.query_row(params![id], |row| {
        Ok(User {
            id: row.get(0).expect("Failed to get ID"),
            attempts: row.get(1).expect("Failed to get attempts"),
            is_won: row.get(2).expect("Failed to get is_won"),
            questions_quantity: row.get(3).expect("Failed to get questions_quantity"),
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

// Функция добавляет пользователя в базу данных, в случае ошибки - возвращает Err()
pub fn try_add_user(user: User, con: &Connection) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO Users (ID, attempts, is_won, questions_quantity) VALUES (?, ?, ?, ?)";

    // Выполняем запрос с помощью метода execute, передавая параметры
    con.execute(
        query,
        params![
            user.id as u64, // Предполагаем, что тип поля ID в базе данных - INTEGER
            user.attempts,
            user.is_won,
            user.questions_quantity,
        ],
    )
    .unwrap_or_else(|e| {
        error!("Ошибка во время подготовки query : {e}");
        panic!();
    });

    Ok(())
}

pub fn update_attempts(
    connection: &Connection,
    user_id: u64,
    attempts: u8,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Изменяю количество попыток для пользователя : {}, новое значение : {}",
        user_id.clone(),
        attempts.clone()
    );

    connection
        .execute(
            "UPDATE users SET attempts = ?1 WHERE id = ?2",
            [attempts as i64, user_id as i64],
        )
        .unwrap();
    Ok(())
}

// Функция для обновления значения поля "is_won" в базе данных
pub fn update_is_won(
    connection: &Connection,
    user_id: u64,
    is_won: bool,
) -> Result<(), &'static dyn Error> {
    connection
        .execute(
            "UPDATE users SET is_won = ?1 WHERE id = ?2",
            [is_won as i64, user_id as i64],
        )
        .unwrap();
    Ok(())
}

// Функция для обновления значения поля "questions_quantity" в базе данных
pub fn update_questions_quantity(
    connection: &Connection,
    user_id: u64,
    questions_quantity: u8,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Изменяю количество заданных вопросов для пользователя : {}, новое значение : {}",
        user_id.clone(),
        user_id.clone()
    );
    connection
        .execute(
            "UPDATE users SET questions_quantity = ?1 WHERE id = ?2",
            [questions_quantity as i64, user_id as i64],
        )
        .unwrap();
    Ok(())
}
