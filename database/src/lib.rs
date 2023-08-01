pub mod init;
pub mod model;

use model::User;

use std::error::Error;
use std::result::Result;

use log::error;
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
    ).unwrap_or_else(|e|{
        error!("Ошибка во время подготовки query : {e}");
        panic!();
    });



    Ok(())
}
