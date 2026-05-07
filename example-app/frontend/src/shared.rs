//! Types & helpers shared across all route pages. Route files are
//! `#[path]`-included into the `frontend` crate by `build.rs`, so they
//! can `use crate::shared::*` to reach this module.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct User {
    #[serde(default)]
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListResponse<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemResponse<T> {
    pub data: T,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiError {
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub errors: serde_json::Value,
}

