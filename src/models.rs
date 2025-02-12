use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Object, Deserialize, Clone, Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}
