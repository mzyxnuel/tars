//! TARS Core — Laravel-inspired framework primitives for Rust.
//!
//! Provides the `Application`, `Router`, `Request`, `Response`,
//! `Controller`, `FormRequest`, and `JsonResource` traits. Wraps axum
//! underneath so developers get a Laravel-like DX with Rust's
//! performance characteristics.

pub mod app;
pub mod binding;
pub mod config;
pub mod controller;
pub mod cors;
pub mod error;
pub mod middleware;
pub mod request;
pub mod resource;
pub mod response;
pub mod route;
pub mod server;
pub mod validation;

pub use app::Application;
pub use binding::{Bindable, NoModel, RouteBindable};
pub use config::Config;
pub use controller::Controller;
pub use cors::Cors;
pub use error::{Error, Result};
pub use middleware::Middleware;
pub use request::Request;
pub use resource::{JsonResource, ResourceCollection};
pub use response::{IntoResponse, Response};
pub use route::{Method, Route, Router};
pub use server::Server;
pub use validation::{FormRequest, Rule, Rules, Validator};

/// Re-exports commonly needed types from underlying crates so user code
/// depends only on `tars_core` in most cases.
pub mod prelude {
    pub use crate::{
        Application, Bindable, Config, Controller, Error, FormRequest, IntoResponse, JsonResource,
        Method, Middleware, NoModel, Request, ResourceCollection, Response, Result, Route,
        RouteBindable, Router, Rule, Rules, Server, Validator,
    };
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
}
