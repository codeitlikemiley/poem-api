use poem::Error;
use poem_openapi::{
    error::ParseRequestPayloadError,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};

#[derive(Object, Serialize, Deserialize)]
pub struct Code<T: ParseFromJSON + ToJSON + Send + Sync> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> Code<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "OK".to_string(),
            data: Some(data),
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            code: 201,
            message: "Created".to_string(),
            data: Some(data),
        }
    }

    pub fn not_found() -> Self {
        Self {
            code: 404,
            message: "Not Found".to_string(),
            data: None,
        }
    }

    pub fn bad_request(msg: String) -> Self {
        Self {
            code: 400,
            message: msg,
            data: None,
        }
    }

    pub fn internal_error(msg: String) -> Self {
        Self {
            code: 500,
            message: msg,
            data: None,
        }
    }
}

#[derive(ApiResponse)]
#[oai(bad_request_handler = "handle_bad_request")]
pub enum Response<T: ParseFromJSON + ToJSON + Send + Sync> {
    #[oai(status = 200)]
    Ok(Json<Code<T>>),
    #[oai(status = 201)]
    Created(Json<Code<T>>),
    #[oai(status = 400)]
    BadRequest(Json<Code<T>>),
    #[oai(status = 404)]
    NotFound(Json<Code<T>>),
    #[oai(status = 500)]
    InternalError(Json<Code<T>>),
}

fn handle_bad_request<T: ParseFromJSON + ToJSON + Send + Sync>(err: Error) -> Response<T> {
    if err.is::<ParseRequestPayloadError>() {
        Response::BadRequest(Json(Code::bad_request(err.to_string())))
    } else {
        Response::InternalError(Json(Code::internal_error(err.to_string())))
    }
}
