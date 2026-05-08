//! Entry point for the TARS frontend. Reads the build-time generated
//! route table from `routes/`, mounts a `FileRouter`, and launches the
//! Dioxus runtime selected by feature flags (web / desktop / mobile).
//!
//! Tailwind is loaded via the Tailwind Play CDN (a `<script>` tag injected
//! by `dioxus::document::Script`). It compiles utility classes in the
//! browser, so no npm/node toolchain is required for the example. For a
//! production-grade build, replace the CDN with a tailwindcss-CLI output
//! file referenced via `Dioxus.toml`'s `[web.resource]` section.

use dioxus::document;
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
        // Inject Tailwind Play CDN. Browser-side compilation — fine for
        // the example app; swap for a CLI-built CSS file in production.
        document::Script { src: "https://cdn.tailwindcss.com" }
        document::Title { "TARS Example" }
        // Keep the bundled tars-ui stylesheet so legacy `tars-*` classes
        // still render. Tailwind utilities work alongside it.
        style { {STYLES} }
        // Tailwind base layout: full-height column with a slate background.
        div { class: "tars-app min-h-screen bg-slate-950 text-slate-100",
            Nav {}
            main { class: "px-4 sm:px-6 lg:px-8 py-6",
                { router.render() }
            }
        }
    }
}

#[component]
fn Nav() -> Element {
    let path = use_router_path();
    // Tailwind classes for the active vs inactive nav links.
    let cls = |route: &str, p: &str| -> &'static str {
        let active = if route == "/" {
            p == "/"
        } else {
            p == route || p.starts_with(&format!("{route}/"))
        };
        if active {
            "px-3 py-1.5 rounded-md bg-indigo-500/15 text-indigo-300 font-medium"
        } else {
            "px-3 py-1.5 rounded-md text-slate-300 hover:text-white hover:bg-slate-800/60 font-medium"
        }
    };
    rsx! {
        nav { class: "flex items-center gap-3 px-6 py-3 bg-slate-900/80 border-b border-slate-800 backdrop-blur",
            span { class: "font-bold tracking-widest text-indigo-300 mr-3", "TARS" }
            Link { class: cls("/", &path).to_string(), to: "/".to_string(), "Home" }
            Link { class: cls("/users", &path).to_string(), to: "/users".to_string(), "Users" }
        }
    }
}

fn not_found() -> Element {
    rsx! {
        section { class: "max-w-xl mx-auto py-16 text-center",
            div { class: "text-7xl font-extrabold text-slate-700 mb-4", "404" }
            h1 { class: "text-2xl font-bold mb-2", "Page not found" }
            p { class: "text-slate-400 mb-6",
                "The page you requested could not be found."
            }
            Link { class: "inline-flex items-center px-5 py-2.5 rounded-md bg-indigo-500 hover:bg-indigo-400 text-white font-medium transition".to_string(),
                to: "/".to_string(), "Go home"
            }
        }
    }
}
