//! Entry point for the TARS frontend. Reads the build-time generated
//! route table from `routes/`, mounts a `FileRouter`, and launches the
//! Dioxus runtime selected by feature flags (web / desktop / mobile).

use dioxus::prelude::*;
use tars_frontend::{launch, use_router_path, FileRouter, Link};
use tars_ui::STYLES;

mod shared;

mod generated {
    include!(concat!(env!("OUT_DIR"), "/generated_routes.rs"));
}

fn main() {
    launch(app);
}

fn app() -> Element {
    let router = FileRouter::new(generated::routes(), not_found);
    rsx! {
        style { {STYLES} }
        div { class: "tars-app",
            Nav {}
            { router.render() }
        }
    }
}

#[component]
fn Nav() -> Element {
    let path = use_router_path();
    let on = |route: &str, p: &str| -> &'static str {
        if route == "/" { if p == "/" { "active" } else { "" } }
        else if p == route || p.starts_with(&format!("{route}/")) { "active" } else { "" }
    };
    rsx! {
        nav { class: "tars-nav",
            span { class: "tars-nav-brand", "TARS" }
            Link { class: on("/", &path).to_string(), to: "/".to_string(), "Home" }
            Link { class: on("/users", &path).to_string(), to: "/users".to_string(), "Users" }
        }
    }
}

fn not_found() -> Element {
    rsx! {
        tars_ui::Container {
            tars_ui::Page { title: "Not found".to_string(),
                tars_ui::Alert { variant: tars_ui::AlertVariant::Warning,
                    "The page you requested could not be found."
                }
            }
        }
    }
}
