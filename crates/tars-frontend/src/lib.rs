//! TARS Frontend — a thin, Vue-inspired wrapper over Dioxus with
//! file-based routing. Compiles to web, desktop, or mobile using Dioxus'
//! existing cross-compilation targets.

pub mod api;
pub mod component;
pub mod router;
pub mod store;

pub use api::Api;
pub use component::{defineComponent, Component, ComponentProps};
pub use router::{FileRouter, Route};
pub use store::{reactive, ref_, Ref};

pub mod prelude {
    pub use crate::{defineComponent, reactive, ref_, Api, Component, ComponentProps, FileRouter, Ref, Route};
    pub use dioxus::prelude::*;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
}
