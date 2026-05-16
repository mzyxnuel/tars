use serde_json::{json, Value};

use crate::response::Response;

/// HTTP transformer for a model. Implementations map a domain value into
/// the public JSON representation the API exposes.
///
/// The convenience constructors [`single`](Self::single) and
/// [`collection`](Self::collection) build a [`Response`] directly so
/// controllers can return the resource with
/// `Ok(UserResource::collection(users))` — no manual mapping or
/// `json!({ "data": … })` wrapping.
pub trait JsonResource: Sized {
    /// The underlying domain type this resource wraps.
    type Model;

    /// Build the resource from a model instance.
    fn from_model(model: Self::Model) -> Self;

    /// Public JSON shape of a single resource. Controllers don't call
    /// this directly; it backs `single` and `collection`.
    fn to_json(&self) -> Value;

    /// Wrap a single model in a `{"data": {...}}` envelope and produce
    /// the `Response`. Mirrors Laravel's `new UserResource($user)`.
    fn single(model: Self::Model) -> Response {
        Response::new(json!({ "data": Self::from_model(model).to_json() }))
    }

    /// Map a vector of models through the resource and wrap them in a
    /// `{"data": [...]}` envelope. Mirrors
    /// `UserResource::collection($users)`.
    fn collection(models: Vec<Self::Model>) -> Response {
        let body: Vec<Value> = models
            .into_iter()
            .map(|m| Self::from_model(m).to_json())
            .collect();
        Response::new(json!({ "data": body }))
    }

    /// Same shape as `single` but with the `201 Created` status. Use
    /// from `store` handlers.
    fn created(model: Self::Model) -> Response {
        Self::single(model).status(axum::http::StatusCode::CREATED)
    }
}

/// Typed wrapper retained for compatibility — most controllers will use
/// [`JsonResource::collection`] directly instead.
pub struct ResourceCollection<R: JsonResource> {
    pub items: Vec<R>,
}

impl<R: JsonResource> ResourceCollection<R> {
    pub fn new(items: Vec<R>) -> Self {
        Self { items }
    }

    pub fn to_json(&self) -> Value {
        let body: Vec<Value> = self.items.iter().map(|r| r.to_json()).collect();
        json!({ "data": body })
    }

    pub fn into_response(self) -> Response {
        Response::new(self.to_json())
    }
}

impl<R: JsonResource> From<ResourceCollection<R>> for Response {
    fn from(c: ResourceCollection<R>) -> Self {
        c.into_response()
    }
}
