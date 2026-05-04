use async_trait::async_trait;
use serde_json::json;
use tars_orm::Factory;

use crate::models::User;

pub struct UserFactory;

#[async_trait]
impl Factory for UserFactory {
    type M = User;

    fn definition(&self) -> serde_json::Value {
        let suffix = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        json!({
            "name": format!("User {}", suffix),
            "email": format!("user{}@example.com", suffix),
        })
    }
}
