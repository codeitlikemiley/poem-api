use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Object, Serialize, Deserialize, Validate)]
pub struct ItemRequest {
    #[validate(length(min = 1, message = "name is too short"))]
    pub name: String,
}

#[derive(Object, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "username must be a valid email"))]
    pub username: String,
}
