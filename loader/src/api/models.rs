use std::collections::HashMap;
use std::error::Error;
use chrono::NaiveDateTime;
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
pub(crate) struct ApiUserCreateRequest {
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) birth_date: NaiveDateTime,
    pub(crate) company_name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiUserDeleteRequest {
    pub(crate) name: String,
    pub(crate) surname: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiProviderCreateRequest {
    pub(crate) name: String,
    pub(crate) user_limit: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiGetUsersByProviderRequest {
    pub(crate) provider: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiGetUsersByProviderResponse {
    pub(crate) data: Vec<ApiUserAccumulated>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiUserAccumulated {
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) birth_date: NaiveDateTime,
    pub(crate) company_name: String,
    pub(crate) provider_name: String,
    pub(crate) accounting_statistics: ApiAccountingStatistics,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub(crate) struct ApiAccountingStatistics {
    pub(crate) counts: HashMap<String, i32>,
    pub(crate) payments: HashMap<String, f32>,
}