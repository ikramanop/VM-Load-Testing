use std::error::Error;
use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket_validation::{Validate};

pub(crate) enum ApiErrors {
    DatabaseError = -127
}

impl ApiErrors {
    pub(crate) fn wrap_anyhow_error(err: anyhow::Error) -> String { format!("Error occurred: {:?}", err) }

    pub(crate) fn wrap_error(err: impl Error) -> String {
        format!("Error occurred: {:?}", err)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiErrorMessage {
    pub(crate) code: i8,
    pub(crate) message: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiOkMessage {
    pub(crate) result: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiCreateLoadTaskRequest {
    pub(crate) endpoints: Vec<String>,
    pub(crate) iterations: i32,
    pub(crate) meta: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiCreateLoadTaskResponse {
    pub(crate) uuid: String,
    pub(crate) endpoints: Vec<String>,
    pub(crate) iterations: i32,
    pub(crate) meta: String,
}