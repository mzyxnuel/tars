//! TARS Frontend — a thin, Vue-inspired wrapper over Dioxus with
//! file-based routing. Compiles to web, desktop, or mobile using Dioxus'
//! existing cross-compilation targets.

pub mod api;
pub mod component;
pub mod link;
pub mod router;
pub mod runtime;
pub mod store;

pub use api::Api;
pub use component::{defineComponent, Component, ComponentProps};
pub use link::Link;
pub use router::{current_path, navigate, use_router_path, FileRouter, Route};
pub use runtime::launch;
pub use store::{reactive, ref_, Ref};

pub mod prelude {
    pub use crate::{
        defineComponent, launch, navigate, reactive, ref_, use_router_path, Api, Component,
        ComponentProps, FileRouter, Link, Ref, Route,
    };
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
}
