//! `/users/:id` — Tailwind-styled user detail page.

use dioxus::prelude::*;
use tars_frontend::prelude::*;
use tars_frontend::Link;

use crate::shared::{ItemResponse, User};

pub fn component() -> Element {
    let params = use_route_params();
    let id = params.get("id").cloned().unwrap_or_default();

    let mut user = use_signal::<Option<User>>(|| None);
    let mut loading = use_signal(|| true);
    let mut error = use_signal::<Option<String>>(|| None);

    let id_for_fetch = id.clone();
    use_effect(move || {
        let id = id_for_fetch.clone();
        spawn(async move {
            loading.set(true);
            error.set(None);
            match Api::default_base()
                .get::<ItemResponse<User>>(&format!("/users/{id}"))
                .await
            {
                Ok(resp) => user.set(Some(resp.data)),
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    });

    let edit_path = format!("/users/{id}/edit");

    rsx! {
        section { class: "max-w-2xl mx-auto",
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold tracking-tight", "User" }
                div { class: "flex gap-2",
                    Link { class: "px-3 py-1.5 rounded-md text-sm text-slate-300 hover:bg-slate-800 transition".to_string(),
                        to: "/users".to_string(), "← Back"
                    }
                    Link { class: "px-3 py-1.5 rounded-md text-sm text-indigo-300 hover:bg-indigo-500/15 transition".to_string(),
                        to: edit_path, "Edit"
                    }
                }
            }

            if let Some(err) = error.read().clone() {
                div { class: "px-4 py-3 rounded-md border border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm",
                    "{err}"
                }
            } else if *loading.read() {
                div { class: "flex items-center gap-2 text-slate-400 text-sm",
                    span { class: "inline-block w-4 h-4 border-2 border-slate-700 border-t-indigo-400 rounded-full animate-spin" }
                    span { "Loading…" }
                }
            } else if let Some(u) = user.read().clone() {
                div { class: "rounded-xl border border-slate-800 bg-slate-900/40 divide-y divide-slate-800",
                    DetailRow { label: "ID", value: u.id.unwrap_or(0).to_string() }
                    DetailRow { label: "Name", value: u.name.clone() }
                    DetailRow { label: "Email", value: u.email.clone() }
                    if let Some(ts) = u.created_at.clone() {
                        DetailRow { label: "Created", value: ts, muted: true }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct DetailRowProps {
    label: &'static str,
    value: String,
    #[props(default = false)]
    muted: bool,
}

#[component]
fn DetailRow(props: DetailRowProps) -> Element {
    let text_cls = if props.muted { "text-slate-400 font-mono text-sm" } else { "text-slate-100" };
    rsx! {
        div { class: "px-5 py-4",
            div { class: "text-xs font-medium text-slate-500 uppercase tracking-wider mb-1", "{props.label}" }
            div { class: "{text_cls}", "{props.value}" }
        }
    }
}
