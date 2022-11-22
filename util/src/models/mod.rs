use std::error::Error;
use std::io::Cursor;
use rocket::http::{ContentType, Status};
use rocket::Request;
use rocket::response::{Responder, Response, Result as RocketResult};
use rocket::serde::json::serde_json;
use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket_validation::{Validate};

pub mod loader;
pub mod manager;

pub type ApiResult<T> = Result<T, ApiErrorMessage>;

#[derive(Debug, Clone)]
pub enum ApiErrors {
    DatabaseError = -127,
    UrlParseError = -126,
    QueueError = -125,
}

impl ApiErrors {
    pub fn wrap_anyhow_error(err: anyhow::Error) -> String { format!("Error occurred: {:?}", err) }

    pub fn wrap_error(err: impl Error) -> String {
        format!("Error occurred: {:?}", err)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiErrorMessage {
    pub code: i8,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiOkMessage {
    pub result: String,
}

impl<'r> Responder<'r, 'static> for ApiErrorMessage {
    fn respond_to(self, _: &'r Request<'_>) -> RocketResult<'static> {
        let err_response = serde_json::to_string(&self).unwrap();

        Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}