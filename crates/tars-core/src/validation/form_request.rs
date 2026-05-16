use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

use crate::binding::Bindable;
use crate::error::{Error, Result};
use crate::request::Request;
use crate::validation::validator::Validator;

/// Laravel's `FormRequest`. Implementations are typed payload structs
/// whose fields hold the validated data, plus a `rules()` function that
/// declares the validation rules. The framework calls
/// `<Self as Bindable>::from_request` automatically when a controller
/// method declares a `FormRequest` parameter — validation failures
/// surface as `Error::Validation` (HTTP 422 with field-level error JSON).
///
/// ```ignore
/// #[derive(serde::Deserialize, serde::Serialize)]
/// pub struct StoreUserRequest {
///     pub name: String,
///     pub email: String,
/// }
///
/// #[async_trait]
/// impl FormRequest for StoreUserRequest {
///     fn rules() -> Vec<(&'static str, &'static str)> {
///         vec![
///             ("name", "required|string|min:2|max:100"),
///             ("email", "required|email|max:255"),
///         ]
///     }
/// }
///
/// async fn store(_: &UserController, req: StoreUserRequest) -> Result<Response> {
///     User::create(req.validated()).await?;
///     ...
/// }
/// ```
pub trait FormRequest: Sized + Send + Sync + DeserializeOwned + Serialize + 'static {
    /// `(field, "rule_string")` pairs. Same Laravel-style DSL the
    /// `Validator` accepts: `"required|email|max:255"`.
    fn rules() -> Vec<(&'static str, &'static str)>;

    /// Authorize the request before validation runs. Defaults to `true`;
    /// override to gate on roles, ownership, etc. Returning `false`
    /// produces a 403.
    fn authorize(_req: &Request) -> bool {
        true
    }

    /// The validated payload as JSON. Mirrors Laravel's
    /// `$request->validated()` — handy when you want to pass the data
    /// straight to `Model::create(req.validated())`.
    fn validated(&self) -> Value {
        serde_json::to_value(self).unwrap_or(Value::Null)
    }
}

/// Every `FormRequest` is `Bindable`: the router validates the body
/// against `rules()`, then deserializes into the typed struct. Validation
/// failures become `Error::Validation`; authorisation failures become
/// `Error::Forbidden`.
#[async_trait]
impl<T: FormRequest> Bindable for T {
    async fn from_request(req: &Request) -> Result<Self> {
        if !T::authorize(req) {
            return Err(Error::Forbidden);
        }
        let validated = match Validator::validate(&req.body, &T::rules()) {
            Ok(v) => v,
            Err(errors) => return Err(Error::Validation(errors)),
        };
        serde_json::from_value(validated).map_err(|e| Error::BadRequest(e.to_string()))
    }
}
