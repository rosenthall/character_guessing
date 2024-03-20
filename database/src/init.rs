use rusqlite::{params, Connection};
use std::result::Result;
use std::time::Duration;
use crate::control;
use crate::control::update_db_connection;
use log::*;

// Creates a new database and table
pub fn create_database_and_table(date: &str) -> Result<Connection, ()> {
    let db_path = format!("data/{}.db", date);
    info!("Database path: {}", db_path.clone());

    let conn = Connection::open(db_path).unwrap_or_else(|e| {
        error!("Error: {e}");
        panic!();
    });

    info!("Creating new table!");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Users (
                  ID INTEGER PRIMARY KEY NOT NULL,
                  attempts INTEGER CHECK (attempts >= 0 AND attempts <= 5),
                  is_won BOOLEAN,
                  questions_quantity INTEGER CHECK (questions_quantity >= 0 AND questions_quantity <= 3)
                  )",
        params![],
    ).unwrap_or_else(|e| {
        error!("Error: {e:#?}");
        panic!();
    });

    Ok(conn)
}

// Updates the database every 24 hours
pub async fn update_db_each_day_service() {
    info!("DB updating Service started!");

    loop {
        tokio::spawn(async {
            let _ = tokio::time::sleep(Duration::from_secs(24 * 60 * 60)).await;
            let formatted_date = control::get_current_formatted_date();

            let new_db = create_database_and_table(&formatted_date).unwrap();
            update_db_connection(new_db).await;
        })
            .await
            .unwrap();
    }
}