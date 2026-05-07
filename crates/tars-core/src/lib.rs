//! TARS Core — Laravel-inspired framework primitives for Rust.
//!
//! Provides the `Application`, `Router`, `Request`, `Response`, `Controller`
//! traits and related helpers. Wraps axum underneath so developers get a
//! Laravel-like DX while keeping the performance characteristics of Rust.

pub mod app;
pub mod config;
pub mod controller;
pub mod cors;
pub mod error;
pub mod middleware;
pub mod request;
pub mod response;
pub mod route;
pub mod server;

pub use app::Application;
pub use config::Config;
pub use controller::Controller;
pub use cors::Cors;
pub use error::{Error, Result};
pub use middleware::Middleware;
pub use request::Request;
pub use response::{IntoResponse, Response};
pub use route::{Method, Route, Router};
pub use server::Server;

/// Re-exports commonly needed types from underlying crates so user code
/// depends only on `tars_core` in most cases.
pub mod prelude {
    pub use crate::{
        Application, Config, Controller, Error, IntoResponse, Middleware,
        Method, Request, Response, Result, Route, Router, Server,
    };
    pub use async_trait::async_trait;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::{json, Value};
}
