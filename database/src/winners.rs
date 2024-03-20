// Importing necessary modules and packages
use crate::model::WinnerEntry;
use log::{error, trace};
use once_cell::sync::Lazy;
use rusqlite::{named_params, params, Connection};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

// Type alias for a database pool
pub type WinnersDbPool = Arc<Mutex<Connection>>;

// Static variable for the database pool
pub static WINNERS_DB: Lazy<WinnersDbPool> = Lazy::new(|| {
    // Open a connection to the database
    let connection = Connection::open("data/winners.db").expect("Failed to open database");

    // Create a table in the database if it does not exist
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS Winners (
                id INTEGER PRIMARY KEY,
                requests INTEGER
            )",
            [],
        )
        .expect("Failed to create table");

    // Return a mutex-guarded connection
    Arc::new(Mutex::new(connection))
});

// Function to add a winner to the database
// If there is an error, it returns Err()
pub fn try_add_winner(user: WinnerEntry, con: &Connection) -> Result<(), Box<dyn Error>> {
    // SQL query to insert a new winner into the database
    let query = "INSERT INTO Winners (id, requests) VALUES (?, ?)";

    // Execute the query with the provided parameters
    con.execute(query, params![user.id, user.requests,]).unwrap_or_else(|e| {
        error!("Error preparing query: {e}");
        panic!();
    });

    Ok(())
}

// Function to get a winner from the database by ID
// If the winner does not exist, it returns None
pub fn try_get_winner(id: u64, con: &Connection) -> Option<WinnerEntry> {
    // SQL query to select a winner by ID
    let query = "SELECT id, requests FROM Winners WHERE ID = ?";

    // Prepare the SQL statement
    let mut stmt = con.prepare(query).unwrap_or_else(|e| {
        error!("Error preparing query: {e}");
        panic!();
    });

    // Execute the query and map the result to a WinnerEntry struct
    let result = stmt.query_row(params![id], |row| {
        Ok(WinnerEntry {
            id: row.get(0).expect("Failed to get id"),
            requests: row.get(1).expect("Failed to get attempts"),
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

// Function to update the number of requests for a winner in the database
pub fn update_winners_requests(
    connection: &Connection,
    user_id: u64,
    attempts: u16,
) -> Result<(), &'static dyn Error> {
    trace!(
        "Updating the number of requests for winner: {}, new value: {}",
        user_id.clone(),
        attempts.clone()
    );

    // Execute the SQL update statement
    connection
        .execute(
            "UPDATE Winners SET requests = :attempts WHERE id = :user_id",
            named_params! {":attempts": attempts, ":user_id": user_id},
        )
        .unwrap();

    Ok(())
}
