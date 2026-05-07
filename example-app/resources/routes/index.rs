//! `/` — landing page. Pure-Tailwind layout that showcases TARS to first-
//! time visitors. Uses utility classes so it blends with the Tailwind CDN
//! mounted in `frontend/src/main.rs` — no `tars-ui` components needed.

use dioxus::prelude::*;
use tars_frontend::Link;

pub fn component() -> Element {
    rsx! {
        section { class: "max-w-5xl mx-auto py-10 sm:py-16",
            // Hero ----------------------------------------------------
            div { class: "text-center space-y-4",
                span { class: "inline-block px-3 py-1 rounded-full bg-indigo-500/10 text-indigo-300 text-xs font-semibold tracking-widest uppercase",
                    "Laravel-in-Rust"
                }
                h1 { class: "text-4xl sm:text-5xl font-extrabold tracking-tight bg-gradient-to-r from-indigo-300 via-sky-300 to-emerald-300 bg-clip-text text-transparent",
                    "Welcome to TARS"
                }
                p { class: "text-slate-400 text-lg max-w-2xl mx-auto",
                    "A full-stack framework for Rust. Controllers, migrations, "
                    "form requests, and a Dioxus frontend — all sharing JSON over the same port."
                }
                div { class: "flex justify-center gap-3 pt-2",
                    Link { class: "inline-flex items-center px-5 py-2.5 rounded-md bg-indigo-500 hover:bg-indigo-400 text-white font-semibold shadow-lg shadow-indigo-500/30 transition".to_string(),
                        to: "/users".to_string(), "Open Users →"
                    }
                    a { class: "inline-flex items-center px-5 py-2.5 rounded-md border border-slate-700 hover:border-slate-500 text-slate-200 font-semibold transition",
                        href: "https://github.com/mzyxnuel/tars", target: "_blank",
                        "GitHub"
                    }
                }
            }

            // Feature grid ---------------------------------------------
            div { class: "grid sm:grid-cols-2 lg:grid-cols-3 gap-4 mt-12",
                FeatureCard {
                    title: "Backend",
                    body: "tars-core + tars-orm wrap axum & sqlx. Controllers, FormRequests, Resources, Migrations, Seeders.",
                }
                FeatureCard {
                    title: "Frontend",
                    body: "tars-frontend wraps Dioxus. File-based routing in resources/routes/, hooks for forms & validation.",
                }
                FeatureCard {
                    title: "JSON-first",
                    body: "Every controller returns JSON. The frontend talks to /api over the same origin — no CORS in dev.",
                }
                FeatureCard {
                    title: "Migrations",
                    body: "Fluent Schema::create(\"users\").id().string(\"name\").timestamps() — runs on boot.",
                }
                FeatureCard {
                    title: "Validation",
                    body: "Laravel-style rule strings (\"required|email|max:255\"). 422 errors auto-render in forms.",
                }
                FeatureCard {
                    title: "Cross-platform",
                    body: "Compiles to web, desktop, and mobile via Dioxus — one codebase, three targets.",
                }
            }
        }
    }
}

#[component]
fn FeatureCard(title: String, body: String) -> Element {
    rsx! {
        div { class: "p-5 rounded-xl border border-slate-800 bg-slate-900/40 hover:bg-slate-900/70 hover:border-slate-700 transition",
            h3 { class: "font-semibold text-slate-100 mb-1", "{title}" }
            p { class: "text-sm text-slate-400 leading-relaxed", "{body}" }
        }
    }
}
