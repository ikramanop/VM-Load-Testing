use sqlx::types::chrono;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    pub(crate) id: i32,
    pub(crate) affiliation_id: i32,
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) birth_date: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UserExt {
    pub(crate) id: i32,
    pub(crate) affiliation_id: i32,
    pub(crate) name: String,
    pub(crate) surname: String,
    pub(crate) birth_date: chrono::NaiveDateTime,
    pub(crate) company_name: String,
    pub(crate) provider_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Affiliation {
    pub(crate) id: i32,
    pub(crate) provider_id: i32,
    pub(crate) company_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Provider {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) user_limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Accounting {
    pub(crate) id: i32,
    pub(crate) user_id: i32,
    pub(crate) amount: i32,
    pub(crate) payment_type: String,
    pub(crate) tax_charged: i32,
    pub(crate) date_paid: chrono::NaiveDateTime,
}