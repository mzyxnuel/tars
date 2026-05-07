//! TARS Frontend — a thin wrapper over Dioxus with file-based routing,
//! a JSON API client, and form-state hooks. Compiles to web, desktop, or
//! mobile through Dioxus' existing cross-compilation targets.

pub mod api;
pub mod component;
pub mod form;
pub mod link;
pub mod router;
pub mod runtime;

pub use api::Api;
pub use component::{define_component, Component, ComponentProps};
pub use form::{use_field, use_validation_errors, Field, ValidationErrors};
pub use link::Link;
pub use router::{
    current_path, extract_params, navigate, path_matches, use_route_params, use_router_path,
    FileRouter, Route,
};
pub use runtime::launch;

// Re-export Dioxus' `use_signal` so callers reach it as
// `tars_frontend::use_signal` without pulling `dioxus::prelude::*`.
pub use dioxus::prelude::use_signal;

pub mod prelude {
    // Note: `Link` is intentionally NOT in the prelude because Dioxus also
    // exports a `Link`. Import `tars_frontend::Link` explicitly when needed.
    pub use crate::{
        define_component, launch, navigate, use_field, use_route_params, use_router_path,
        use_validation_errors, Api, Component, ComponentProps, Field, FileRouter, Route,
        ValidationErrors,
    };
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
}
