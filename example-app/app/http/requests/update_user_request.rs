use serde::{Deserialize, Serialize};
use tars_core::FormRequest;

/// Form request for `PUT /users/:id`. Same rule shape as store but every
/// field is optional — partial updates are allowed.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

impl FormRequest for UpdateUserRequest {
    fn rules() -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "nullable|string|min:2|max:100"),
            ("email", "nullable|email|max:255"),
        ]
    }

    /// Override to drop `None` fields so we never write `null` over an
    /// existing column.
    fn validated(&self) -> serde_json::Value {
        let mut map = serde_json::Map::new();
        if let Some(n) = &self.name { map.insert("name".into(), n.clone().into()); }
        if let Some(e) = &self.email { map.insert("email".into(), e.clone().into()); }
        serde_json::Value::Object(map)
    }
}
