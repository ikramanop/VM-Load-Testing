use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Task {
    pub(crate) id: i32,
    pub(crate) uuid: String,
    pub(crate) endpoints: Vec<String>,
    pub(crate) status: String,
    pub(crate) iterations: i32,
    pub(crate) meta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Job {
    pub(crate) id: i32,
    pub(crate) task_id: i32,
    pub(crate) uuid: String,
    pub(crate) endpoint: String,
    pub(crate) status: String,
    pub(crate) iteration: i32,
    pub(crate) updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Statistic {
    pub(crate) id: i32,
    pub(crate) task_id: i32,
    pub(crate) response: f32,
}