// Importing necessary modules and packages
use crate::init::create_database_and_table;
use async_trait::async_trait;
use chrono::prelude::*;
use log::info;
use once_cell::sync::Lazy;
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

// Function to get the current date in the format "%y-%m-%d"
pub(crate) fn get_current_formatted_date() -> String {
    let formatted_date = {
        let utc_now: DateTime<Utc> = Utc::now();
        let date = utc_now.date_naive();

        date.format("%y-%m-%d").to_string()
    };

    formatted_date
}

// Trait for controlling the database
#[async_trait]
trait ControlDatabase {
    // Constructor
    fn from(c: Arc<Mutex<Connection>>) -> Self;

    // Function for the first initialization of the database
    fn init() -> Self;

    // Function to get a connection to the database
    async fn get_connection(&self) -> MutexGuard<Connection>;

    // Function to update the current connection to the database,
    // in particular, it sets a new date and creates a new file if the date has changed
    async fn update(&mut self);
}

// Type alias for a database handler
pub type DatabaseHandler = Arc<Mutex<Connection>>;

// Implementing the ControlDatabase trait for the DatabaseHandler type
#[async_trait]
impl ControlDatabase for DatabaseHandler {
    // Function to create a new DatabaseHandler from a connection
    fn from(c: Arc<Mutex<Connection>>) -> Self {
        c
    }

    // Function to create a new DatabaseHandler
    fn init() -> Self {
        let formatted_date = get_current_formatted_date();

        let con = create_database_and_table(&formatted_date).unwrap();

        Arc::new(Mutex::new(con))
    }

    // Function to get a connection to the database
    async fn get_connection(&self) -> MutexGuard<Connection> {
        let connection = self.lock().await;

        connection
    }

    // Function to update the current connection to the database
    async fn update(&mut self) {
        let formatted_date = get_current_formatted_date();

        let con = create_database_and_table(&formatted_date).unwrap();
        self = &mut ControlDatabase::from(Arc::new(Mutex::new(con)));
    }
}

// Static variable for the database handler
pub static DATABASE_HANDLER: Lazy<DatabaseHandler> = Lazy::new(|| {
    DatabaseHandler::init()
});

// Function to update the database connection
pub(crate) async fn update_db_connection(con: Connection) {
    *DATABASE_HANDLER.lock().await = con;
    info!("Updated db connection!");
}