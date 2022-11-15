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
pub(crate) struct QueueItem {
    pub(crate) id: i32,
    pub(crate) task_id: i32,
    pub(crate) endpoint: String,
    pub(crate) response: Option<f64>,
    pub(crate) status: i32,
}