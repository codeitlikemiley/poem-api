use poem_openapi::Object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct ItemRequest {
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
}

#[derive(Object)]
pub struct LoginRequest {
    #[oai(validator(min_length = 3, max_length = 60,))]
    pub username: String,
}
