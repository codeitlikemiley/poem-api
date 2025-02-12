use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Object, Serialize, Deserialize)]
pub struct ItemRequest {
    #[oai(validator(min_length = 3, max_length = 60))]
    pub name: String,
}

#[derive(Object, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "username must be a valid email"))]
    pub username: String,
}
