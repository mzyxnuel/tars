//! Entry point for the TARS frontend. Reads the build-time generated route
//! table from `resources/routes/`, mounts a `FileRouter`, and launches the
//! Dioxus runtime selected by feature flags (web / desktop / mobile).

use dioxus::prelude::*;
use tars_frontend::{launch, FileRouter, Link};

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_routes.rs"));
}

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn app() -> Element {
    let router = FileRouter::new(generated::routes(), not_found);
    rsx! {
        div { class: "tars-app",
            Nav {}
            { router.render() }
        }
    }
}

#[component]
fn Nav() -> Element {
    rsx! {
        nav {
            Link { to: "/".to_string(), "Home" }
            " | "
            Link { to: "/users".to_string(), "Users" }
        }
    }
}

fn not_found() -> Element {
    rsx! { div { h1 { "404" } p { "Page not found." } } }
}
