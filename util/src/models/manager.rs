use serde::{Serialize, Deserialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::okapi::schemars;
use rocket_validation::{Validate};

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