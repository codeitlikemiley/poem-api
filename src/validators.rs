use poem_openapi::Object;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Object)]
pub struct Validation {
    errors: serde_json::Value,
}

impl From<ValidationErrors> for Validation {
    fn from(errors: ValidationErrors) -> Self {
        Self {
            errors: serde_json::to_value(errors)
                .unwrap_or_else(|_| json!({"error": "Invalid input"})),
        }
    }
}
