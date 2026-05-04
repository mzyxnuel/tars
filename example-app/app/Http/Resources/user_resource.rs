use serde_json::{json, Value};

use crate::models::User;

/// API representation of a `User`. Lets the controller keep its output
/// shape separate from the DB model.
pub struct UserResource {
    pub user: User,
}

impl UserResource {
    pub fn from_user(user: User) -> Self {
        Self { user }
    }

    pub fn to_json(&self) -> Value {
        json!({
            "id": self.user.id,
            "name": self.user.name,
            "email": self.user.email,
        })
    }
}
