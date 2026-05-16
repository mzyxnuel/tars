//! Validation primitives — rule parser, validator, and `FormRequest`.
//!
//! Folded into `tars-core` so the blanket `Bindable` impl for
//! `FormRequest` doesn't trip Rust's orphan rule.

pub mod form_request;
pub mod rule;
pub mod validator;

pub use form_request::FormRequest;
pub use rule::{parse_rules, Rule, Rules};
pub use validator::Validator;
