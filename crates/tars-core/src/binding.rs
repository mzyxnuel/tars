//! Extractor traits used by controller methods.
//!
//! - [`Bindable`] — turn a [`Request`](crate::Request) into a typed
//!   parameter. `()` is a no-op extractor; `tars-validation`'s
//!   `FormRequest` gets a blanket impl that runs validation +
//!   deserialization.
//! - [`RouteBindable`] — resolve a `:id` path parameter into a model.
//!   `tars-orm`'s `Model` gets a blanket impl that delegates to
//!   `Model::find`.

use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::request::Request;

/// Type that can be built from a `Request`. Implementations decode the
/// body, query, headers, etc. into a typed value, returning an
/// `Error::Validation` / `Error::BadRequest` on failure.
#[async_trait]
pub trait Bindable: Sized + Send + Sync + 'static {
    async fn from_request(req: &Request) -> Result<Self>;
}

#[async_trait]
impl Bindable for () {
    async fn from_request(_req: &Request) -> Result<Self> {
        Ok(())
    }
}

#[async_trait]
impl Bindable for Request {
    async fn from_request(req: &Request) -> Result<Self> {
        Ok(req.clone())
    }
}

/// Type that can be resolved from a `:id` path parameter. Mirrors
/// Laravel's implicit route-model binding — the framework calls
/// `route_bind("42")` to fetch the model, returns 404 if it can't.
#[async_trait]
pub trait RouteBindable: Sized + Send + Sync + 'static {
    /// Look up the bound value by its primary-key-like id string. Return
    /// `Ok(None)` to surface a 404 to the client; `Err(...)` for other
    /// failure modes.
    async fn route_bind(id: &str) -> Result<Option<Self>>;

    /// Convenience helper used by the router — wraps `route_bind` and
    /// turns `Ok(None)` into `Error::NotFound`.
    async fn require_bind(id: &str) -> Result<Self> {
        Self::route_bind(id).await?.ok_or(Error::NotFound)
    }
}

/// Sentinel for controllers that don't have a meaningful "model" — e.g.
/// a one-off action controller. Any `:id` lookup against this type
/// errors out with `Error::NotFound`.
pub struct NoModel;

#[async_trait]
impl RouteBindable for NoModel {
    async fn route_bind(_id: &str) -> Result<Option<Self>> {
        Ok(None)
    }
}
