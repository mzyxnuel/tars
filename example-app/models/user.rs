use serde::{Deserialize, Serialize};
use tars_orm::Model;

/// User model — shared between frontend and backend via the `/models`
/// directory. Serde-compatible so the struct is trivially round-trippable
/// as JSON. The `created_at`/`updated_at` fields are filled by the ORM
/// automatically on `create` and `update` (`Model::uses_timestamps()`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

impl Model for User {
    fn table() -> &'static str {
        "users"
    }
}

tars_orm::bind_model!(User);
