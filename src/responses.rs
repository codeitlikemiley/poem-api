use crate::models::Item;
use poem_openapi::{
    payload::{Json, PlainText},
    ApiResponse,
};

pub mod login {
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(PlainText<String>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 500)]
        InternalError,
        #[oai(status = 400)]
        BadRequest(PlainText<String>),
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
        BadRequest(PlainText<String>),
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
        BadRequest(PlainText<String>),
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
        BadRequest(PlainText<String>),
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
        BadRequest(PlainText<String>),
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
    use super::*;

    #[derive(ApiResponse)]
    pub enum Response {
        #[oai(status = 200)]
        Ok(PlainText<String>),
    }

    #[derive(ApiResponse)]
    #[oai(bad_request_handler = "bad_request_handler")]
    pub enum Error {
        #[oai(status = 404)]
        NotFound,
        #[oai(status = 400)]
        BadRequest(PlainText<String>),
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
