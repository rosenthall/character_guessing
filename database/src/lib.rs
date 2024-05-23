pub mod control;
pub mod init;
pub mod model;
pub mod winners;
// Importing necessary modules and packages
use log::{error, trace};
use model::UserDbEntry;
use rusqlite::{params, Connection};
use std::{error::Error, result::Result};

// Function to check if a user exists in the database
// If the user exists, it returns a UserDbEntry struct, otherwise it returns
// None
pub fn check_user(id: u64, con: &Connection) -> Option<model::UserDbEntry> {
    // SQL query to select a user by ID
    let query = "SELECT ID, attempts, is_won, questions_quantity FROM Users WHERE ID = ?";
    let id = id as i64;

    // Prepare the SQL statement
    let mut stmt = con.prepare(query).unwrap_or_else(|e| {
        error!("Error preparing query: {e}");
        panic!();
    });

    // Execute the query and map the result to a UserDbEntry struct
    let result = stmt.query_row(params![id], |row| {
        Ok(UserDbEntry {
            id: row.get(0).expect("Failed to get ID"),
            attempts: row.get(1).expect("Failed to get attempts"),
            is_won: row.get(2).expect("Failed to get is_won"),
            questions_quantity: row.get(3).expect("Failed to get questions_quantity"),
        })
    });

    // Handle the result of the query
    match result {
        Ok(user) => Some(user),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(err) => {
            eprintln!("Error fetching user: {:?}", err);
            None
        }
    }
}

// Function to get a list of all users who have won today
pub fn get_winning_user_ids(con: &Connection) -> Option<Vec<u64>> {
    // Prepare the SQL statement
    let mut query = con.prepare("SELECT ID FROM Users WHERE is_won = 1").unwrap();

    // Execute the query and collect the result into a vector
    let user_ids = query.query_map([], |row| row.get(0)).unwrap().filter_map(Result::ok).collect();

    Some(user_ids)
}

// Function to add a user to the database
// If there is an error, it returns Err()
pub fn try_add_user(user: UserDbEntry, con: &Connection) -> Result<(), Box<dyn Error>> {
    // SQL query to insert a new user into the database
    let query = "INSERT INTO Users (ID, attempts, is_won, questions_quantity) VALUES (?, ?, ?, ?)";

    // Execute the query with the provided parameters
    con.execute(query, params![user.id, user.attempts, user.is_won, user.questions_quantity,])
        .unwrap_or_else(|e| {
            error!("Error preparing query: {e}");
            panic!();
        });

    Ok(())
}

// Function to update the number of attempts for a user
pub fn update_attempts(
    connection: &Connection,
    user_id: u64,
    attempts: u8,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Updating the number of attempts for user: {}, new value: {}",
        user_id.clone(),
        attempts.clone()
    );

    // Execute the SQL update statement
    connection
        .execute("UPDATE users SET attempts = ?1 WHERE id = ?2", [attempts as i64, user_id as i64])
        .unwrap();
    Ok(())
}

// Function to update the "is_won" field for a user in the database
pub fn update_is_won(
    connection: &Connection,
    user_id: u64,
    is_won: bool,
) -> Result<(), &'static dyn Error> {
    // Execute the SQL update statement
    connection
        .execute("UPDATE users SET is_won = ?1 WHERE id = ?2", [is_won as i64, user_id as i64])
        .unwrap();
    Ok(())
}

// Function to update the "questions_quantity" field for a user in the database
pub fn update_questions_quantity(
    connection: &Connection,
    user_id: u64,
    questions_quantity: u8,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Updating the number of questions asked by user: {}, new value: {}",
        user_id.clone(),
        questions_quantity.clone()
    );
    // Execute the SQL update statement
    connection
        .execute(
            "UPDATE users SET questions_quantity = ?1 WHERE id = ?2",
            [questions_quantity as i64, user_id as i64],
        )
        .unwrap();
    Ok(())
}
