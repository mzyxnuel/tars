//! TARS Frontend — a thin, Vue-inspired wrapper over Dioxus with
//! file-based routing. Compiles to web, desktop, or mobile using Dioxus'
//! existing cross-compilation targets.

pub mod api;
pub mod component;
pub mod form;
pub mod link;
pub mod router;
pub mod runtime;
pub mod store;

pub use api::Api;
pub use component::{defineComponent, Component, ComponentProps};
pub use form::{use_field, use_validation_errors, Field, ValidationErrors};
pub use link::Link;
pub use router::{
    current_path, extract_params, navigate, path_matches, use_route_params, use_router_path,
    FileRouter, Route,
};
pub use runtime::launch;
pub use store::{reactive, ref_, Ref};

pub mod prelude {
    // Note: `Link` is intentionally NOT in the prelude because Dioxus also
    // exports a `Link`. Import `tars_frontend::Link` explicitly when needed.
    pub use crate::{
        defineComponent, launch, navigate, reactive, ref_, use_field, use_route_params,
        use_router_path, use_validation_errors, Api, Component, ComponentProps, Field, FileRouter,
        Ref, Route, ValidationErrors,
    };
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
}
