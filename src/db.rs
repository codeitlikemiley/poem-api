use poem::http::StatusCode;
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

pub async fn load_db() -> Result<Db, std::io::Error> {
    let contents = match read_to_string(DB_FILE).await {
        Ok(data) => data,
        Err(_) => {
            let db = Database::default();
            let db_json = to_string_pretty(&db).unwrap();
            write(DB_FILE, db_json).await?;
            return Ok(Arc::new(Mutex::new(db)));
        }
    };

    let db = match from_str::<Database>(&contents) {
        Ok(parsed) => parsed,
        Err(_) => {
            let db = Database::default();
            let db_json = to_string_pretty(&db).unwrap();
            write(DB_FILE, db_json).await?;
            db
        }
    };

    Ok(Arc::new(Mutex::new(db)))
}

pub async fn save_db(db_guard: &Database) -> poem::Result<()> {
    let json = to_string_pretty(db_guard).map_err(|_| {
        let message = format!("Failed to Read File from {}", DB_FILE);
        poem::Error::from_string(message, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    write(DB_FILE, json).await.map_err(|_| {
        let message = format!("Failed to write on {}", DB_FILE);
        poem::Error::from_string(message, StatusCode::INTERNAL_SERVER_ERROR)
    })?;

    Ok(())
}
