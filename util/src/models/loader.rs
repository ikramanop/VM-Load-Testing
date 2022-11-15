use std::collections::HashMap;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket_validation::{Validate};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiUserCreateRequest {
    pub name: String,
    pub surname: String,
    pub birth_date: NaiveDateTime,
    pub company_name: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiUserDeleteRequest {
    pub name: String,
    pub surname: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiProviderCreateRequest {
    pub name: String,
    pub user_limit: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiGetUsersByProviderRequest {
    pub provider: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiGetUsersByProviderResponse {
    pub data: Vec<ApiUserAccumulated>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiUserAccumulated {
    pub name: String,
    pub surname: String,
    pub birth_date: NaiveDateTime,
    pub company_name: String,
    pub provider_name: String,
    pub accounting_statistics: ApiAccountingStatistics,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiAccountingStatistics {
    pub counts: HashMap<String, i32>,
    pub payments: HashMap<String, f32>,
}