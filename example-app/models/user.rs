use serde::{Deserialize, Serialize};
use tars_orm::Model;

/// User model — shared between frontend and backend via the `/models`
/// directory. Serde-compatible so the struct is trivially round-trippable
/// as JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
}

impl Model for User {
    fn table() -> &'static str {
        "users"
    }
}
