use serde_json::{json, Value};
use tars_core::JsonResource;

use crate::models::User;

/// API representation of a `User`. Use via `JsonResource`'s
/// `single` / `collection` / `created` constructors — they wrap the
/// mapped JSON in a `{"data": ...}` envelope and return a `Response`
/// directly.
///
/// ```ignore
/// async fn index(&self) -> Result<Response> {
///     Ok(UserResource::collection(User::all().await?))
/// }
/// ```
pub struct UserResource {
    pub user: User,
}

impl JsonResource for UserResource {
    type Model = User;

    fn from_model(user: User) -> Self {
        Self { user }
    }

    fn to_json(&self) -> Value {
        json!({
            "id": self.user.id,
            "name": self.user.name,
            "email": self.user.email,
            "created_at": self.user.created_at,
            "updated_at": self.user.updated_at,
        })
    }
}
