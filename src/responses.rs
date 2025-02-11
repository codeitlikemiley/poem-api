use poem::Error;
use poem_openapi::{
    error::ParseRequestPayloadError,
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};
use serde::{Deserialize, Serialize};

pub trait ApiData: ParseFromJSON + ToJSON + Send + Sync {}
impl<T: ParseFromJSON + ToJSON + Send + Sync> ApiData for T {}

#[derive(Object, Serialize, Deserialize)]
pub struct Schema<T: ApiData> {
    code: u16,
    message: String,
    data: Option<T>,
}

impl<T: ApiData> Schema<T> {
    pub fn ok(msg: String) -> Self {
        Self {
            code: 200,
            message: msg,
            data: None,
        }
    }

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

    pub fn deleted() -> Self {
        Self {
            code: 204,
            message: "".to_string(),
            data: None,
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
pub enum Responses<T: ApiData> {
    #[oai(status = 200)]
    Ok(Json<Schema<T>>),
    #[oai(status = 201)]
    Created(Json<Schema<T>>),
    #[oai(status = 204)]
    Deleted(Json<Schema<T>>),
    #[oai(status = 400)]
    BadRequest(Json<Schema<T>>),
    #[oai(status = 404)]
    NotFound(Json<Schema<T>>),
    #[oai(status = 500)]
    InternalError(Json<Schema<T>>),
}

fn handle_bad_request<T: ParseFromJSON + ToJSON + Send + Sync>(err: Error) -> Responses<T> {
    if err.is::<ParseRequestPayloadError>() {
        Responses::BadRequest(Json(Schema::bad_request(err.to_string())))
    } else {
        Responses::InternalError(Json(Schema::internal_error(err.to_string())))
    }
}

pub struct Return<T: ParseFromJSON + ToJSON + Send + Sync>(Responses<T>);

impl<T: ParseFromJSON + ToJSON + Send + Sync> Return<T> {
    pub fn success(data: T) -> Responses<T> {
        Responses::Ok(Json(Schema::success(data)))
    }

    pub fn ok(message: impl Into<String>) -> Responses<T> {
        Responses::Ok(Json(Schema::ok(message.into())))
    }

    pub fn deleted() -> Responses<T> {
        Responses::Deleted(Json(Schema::deleted()))
    }

    pub fn created(data: T) -> Responses<T> {
        Responses::Created(Json(Schema::created(data)))
    }

    pub fn not_found() -> Responses<T> {
        Responses::NotFound(Json(Schema::not_found()))
    }

    pub fn bad_request(message: impl Into<String>) -> Responses<T> {
        Responses::BadRequest(Json(Schema::bad_request(message.into())))
    }

    pub fn internal_error(message: impl Into<String>) -> Responses<T> {
        Responses::InternalError(Json(Schema::internal_error(message.into())))
    }

    pub fn from_error(err: Error) -> Responses<T> {
        if err.is::<ParseRequestPayloadError>() {
            Self::bad_request(err.to_string())
        } else {
            Self::internal_error(err.to_string())
        }
    }
}
