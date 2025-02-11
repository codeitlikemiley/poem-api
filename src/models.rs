use poem_openapi::{Enum, Object, Tags};
use serde::{Deserialize, Serialize};

#[derive(Object, Deserialize, Clone, Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
}

#[derive(Enum)]
enum PetStatus {
    Available,
    Pending,
    Sold,
}
