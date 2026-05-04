use async_trait::async_trait;
use serde::de::DeserializeOwned;
use tars_core::error::{Error, Result};
use tars_core::request::Request;

use crate::validator::Validator;

/// Laravel's FormRequest — subclass (well, trait-impl) that declares rules
/// and optionally transforms the input before it reaches the controller.
#[async_trait]
pub trait FormRequest: Send + Sync {
    /// Field → rule-string map: `("email", "required|email")`.
    fn rules() -> Vec<(&'static str, &'static str)>;

    /// Authorize the request — override to implement gates/policies.
    fn authorize(_req: &Request) -> bool {
        true
    }

    /// Pull the validated JSON payload out of the request. Returns a TARS
    /// `Error::Validation` on failure — automatically JSON-serialised by
    /// the framework error handler.
    async fn validated(req: &Request) -> Result<serde_json::Value> {
        if !Self::authorize(req) {
            return Err(Error::Forbidden);
        }
        let rules = Self::rules();
        match Validator::validate(&req.body, &rules) {
            Ok(v) => Ok(v),
            Err(errors) => Err(Error::Validation(errors)),
        }
    }

    /// Like `validated` but parses directly into a typed struct.
    async fn typed<T: DeserializeOwned>(req: &Request) -> Result<T> {
        let v = Self::validated(req).await?;
        serde_json::from_value(v).map_err(|e| Error::BadRequest(e.to_string()))
    }
}
