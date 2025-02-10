use poem_openapi::Object;

#[derive(Object)]
pub struct DeleteItemResponse {
    pub message: String,
}

impl Default for DeleteItemResponse {
    fn default() -> Self {
        Self {
            message: "Item deleted successfully".to_string(),
        }
    }
}
