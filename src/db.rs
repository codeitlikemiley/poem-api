use poem::{http::StatusCode, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::sync::Arc;
use tokio::{
    fs::{read_to_string, write},
    sync::Mutex,
};

use crate::models::Item;

pub type Db = Arc<Mutex<Database>>;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Database {
    pub last_id: u32,
    pub items: Vec<Item>,
}

const DB_FILE: &str = "data.json";

pub async fn load_db() -> Result<Db> {
    let contents = match read_to_string(DB_FILE).await {
        Ok(data) => data,
        Err(_) => {
            // If the file does not exist, initialize an empty database
            let db = Database::default();
            let db_json = to_string_pretty(&db).unwrap(); // Safe because we control the struct
            write(DB_FILE, db_json).await.map_err(|_| {
                Error::from_string(
                    format!("Failed to create database file: {}", DB_FILE),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;
            return Ok(Arc::new(Mutex::new(db)));
        }
    };

    let db = match from_str::<Database>(&contents) {
        Ok(parsed) => parsed,
        Err(_) => {
            // If parsing fails, initialize a new empty database
            let db = Database::default();
            let db_json = to_string_pretty(&db).unwrap();
            write(DB_FILE, db_json).await.map_err(|_| {
                Error::from_string(
                    format!("Failed to reset database file: {}", DB_FILE),
                    StatusCode::INTERNAL_SERVER_ERROR,
                )
            })?;
            db
        }
    };

    Ok(Arc::new(Mutex::new(db)))
}

pub async fn save_db(db_guard: &Database) -> Result<()> {
    let json = to_string_pretty(db_guard).map_err(|_| {
        let message = format!("Failed to Read File from {}", DB_FILE);
        Error::from_string(message, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    write(DB_FILE, json).await.map_err(|_| {
        let message = format!("Failed to write on {}", DB_FILE);
        Error::from_string(message, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    Ok(())
}
