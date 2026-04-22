//! TARS Validation — Laravel-style validation rules and form requests.

pub mod form_request;
pub mod rule;
pub mod validator;

pub use form_request::FormRequest;
pub use rule::{Rule, Rules};
pub use validator::Validator;

pub mod prelude {
    pub use crate::{FormRequest, Rule, Rules, Validator};
    pub use async_trait::async_trait;
}
