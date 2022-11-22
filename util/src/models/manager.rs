use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket_validation::{Validate};

pub const STATUS_PENDING: &str = "pending";
pub const STATUS_DONE: &str = "done";

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiCreateLoadTaskRequest {
    pub endpoints: Vec<String>,
    pub iterations: i32,
    pub meta: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiCreateLoadTaskResponse {
    pub uuid: String,
    pub endpoints: Vec<String>,
    pub iterations: i32,
    pub meta: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiGetTaskSummaryRequest {
    pub uuid: String,
    pub base_avg: Option<f64>,
    pub three_sigma_percent: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct ApiGetTaskSummaryResponse {
    pub uuid: String,
    pub endpoints: Vec<String>,
    pub iterations: i32,
    pub meta: String,
    pub statistics: HashMap<String, f64>,
}