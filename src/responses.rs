use crate::models::Item;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};

use crate::validators::Validation;

pub type CustomMessage = PlainText<String>;

pub mod login {

    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(CustomMessage),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 500)]
        InternalError,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
        #[oai(status = 422)]
        ValidationErrors(Json<Validation>),
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid credentials: {}",
            err.to_string()
        )))
    }
}

pub mod fetch_items {
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(Json<Vec<Item>>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 500)]
        InternalError,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid request for fetching items: {}",
            err.to_string()
        )))
    }
}

pub mod find_item {
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(Json<Item>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 404)]
        NotFound,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid request for finding item: {}",
            err.to_string()
        )))
    }
}

pub mod create_item {
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 201)]
        Created(Json<Item>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 500)]
        InternalError,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
        #[oai(status = 422)]
        ValidationErrors(Json<Validation>),
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid request for creating item: {}",
            err.to_string()
        )))
    }
}

pub mod modify_item {
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(Json<Item>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 404)]
        NotFound,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
        #[oai(status = 422)]
        ValidationErrors(Json<Validation>),
        #[oai(status = 500)]
        InternalError,
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid request for modifying item: {}",
            err.to_string()
        )))
    }
}

pub mod remove_item {
    use poem_openapi::Object;
    use serde::Serialize;

    use super::*;

    #[derive(Object, Serialize)]
    pub struct DeletedMessage {
        message: String,
    }

    impl DeletedMessage {
        pub fn new() -> Self {
            Self {
                message: "Item deleted successfully".to_string(),
            }
        }
    }

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(Json<DeletedMessage>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 404)]
        NotFound,
        #[oai(status = 400)]
        BadRequest(CustomMessage),
        #[oai(status = 500)]
        InternalError,
    }

    fn bad_request_handler(err: poem::Error) -> Error {
        Error::BadRequest(PlainText(format!(
            "Invalid request for removing item: {}",
            err.to_string()
        )))
    }
}
