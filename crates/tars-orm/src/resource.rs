use serde::Serialize;
use serde_json::Value;

/// Equivalent of Laravel's `JsonResource`. Implementations map a model into
/// a JSON representation — typically used in controllers via
/// `UserResource::from(user).to_json()`.
pub trait JsonResource {
    type M;

    fn from_model(model: Self::M) -> Self;

    fn to_json(&self) -> Value;
}

/// Resource collection wrapper — Laravel's `UserResource::collection($users)`.
pub struct ResourceCollection<R: JsonResource> {
    pub items: Vec<R>,
}

impl<R: JsonResource + Serialize> ResourceCollection<R> {
    pub fn new(items: Vec<R>) -> Self {
        Self { items }
    }

    pub fn to_json(&self) -> Value {
        Value::Array(self.items.iter().map(|r| r.to_json()).collect())
    }
}
