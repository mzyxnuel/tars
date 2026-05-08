//! `/users` — index. Lists every user in a Tailwind-styled table with
//! view / edit / delete actions and a button to open the create form.

use dioxus::prelude::*;
use tars_frontend::prelude::*;
use tars_frontend::Link;

use crate::shared::{ListResponse, User};

pub fn component() -> Element {
    let mut users = use_signal::<Vec<User>>(Vec::new);
    let mut loading = use_signal(|| true);
    let mut error = use_signal::<Option<String>>(|| None);
    let mut reload_token = use_signal(|| 0u32);

    use_effect(move || {
        let _ = reload_token.read();
        spawn(async move {
            loading.set(true);
            error.set(None);
            match Api::default_base().get::<ListResponse<User>>("/users").await {
                Ok(resp) => users.set(resp.data),
                Err(e) => error.set(Some(e)),
            }
            loading.set(false);
        });
    });

    let count = users.read().len();

    rsx! {
        section { class: "max-w-5xl mx-auto",
            // Header --------------------------------------------------
            div { class: "flex items-center justify-between mb-6",
                div {
                    h1 { class: "text-2xl font-bold tracking-tight", "Users" }
                    p { class: "text-sm text-slate-400", "{count} total" }
                }
                Link { to: "/users/create".to_string(),
                    span { class: "inline-flex items-center px-4 py-2 rounded-md bg-indigo-500 hover:bg-indigo-400 text-white font-medium text-sm shadow-sm transition",
                        "+ Create user"
                    }
                }
            }

            // Error banner --------------------------------------------
            if let Some(e) = error.read().clone() {
                div { class: "mb-4 px-4 py-3 rounded-md border border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm",
                    "{e}"
                }
            }

            // Body ----------------------------------------------------
            if *loading.read() {
                div { class: "flex items-center gap-2 text-slate-400 text-sm",
                    span { class: "inline-block w-4 h-4 border-2 border-slate-700 border-t-indigo-400 rounded-full animate-spin" }
                    span { "Loading…" }
                }
            } else if users.read().is_empty() {
                div { class: "rounded-xl border border-slate-800 bg-slate-900/40 p-10 text-center text-slate-400",
                    "No users yet. Click \"Create user\" to add one."
                }
            } else {
                div { class: "rounded-xl border border-slate-800 bg-slate-900/40 overflow-hidden",
                    table { class: "w-full text-sm",
                        thead { class: "bg-slate-900/70 text-slate-400 text-xs uppercase tracking-wider",
                            tr {
                                th { class: "text-left px-4 py-3", "ID" }
                                th { class: "text-left px-4 py-3", "Name" }
                                th { class: "text-left px-4 py-3", "Email" }
                                th { class: "text-right px-4 py-3", "" }
                            }
                        }
                        tbody { class: "divide-y divide-slate-800",
                            for u in users.read().iter().cloned() {
                                UserRow { user: u, on_deleted: move |_| reload_token += 1 }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct UserRowProps {
    user: User,
    on_deleted: EventHandler<()>,
}

#[component]
fn UserRow(props: UserRowProps) -> Element {
    let user = props.user.clone();
    let id_str = user.id.map(|n| n.to_string()).unwrap_or_default();
    let id_for_delete = id_str.clone();
    let mut deleting = use_signal(|| false);
    let mut delete_error = use_signal::<Option<String>>(|| None);

    let on_delete = move |_evt: MouseEvent| {
        let id = id_for_delete.clone();
        let on_done = props.on_deleted;
        spawn(async move {
            deleting.set(true);
            delete_error.set(None);
            match Api::default_base().delete(&format!("/users/{id}")).await {
                Ok(_) => on_done.call(()),
                Err(e) => delete_error.set(Some(e)),
            }
            deleting.set(false);
        });
    };

    let action = "px-2.5 py-1 rounded-md text-xs font-medium transition";

    rsx! {
        tr { class: "hover:bg-slate-900/70 transition",
            td { class: "px-4 py-3 text-slate-400 font-mono", "{id_str}" }
            td { class: "px-4 py-3 font-medium text-slate-100", "{user.name}" }
            td { class: "px-4 py-3 text-slate-300", "{user.email}" }
            td { class: "px-4 py-3",
                div { class: "flex justify-end gap-2",
                    Link { class: format!("{action} text-slate-300 hover:bg-slate-800"),
                        to: format!("/users/{}", id_str), "View"
                    }
                    Link { class: format!("{action} text-indigo-300 hover:bg-indigo-500/15"),
                        to: format!("/users/{}/edit", id_str), "Edit"
                    }
                    button {
                        class: format!("{action} text-rose-300 hover:bg-rose-500/15 disabled:opacity-50 disabled:cursor-not-allowed"),
                        disabled: *deleting.read(),
                        onclick: on_delete,
                        if *deleting.read() { "Deleting…" } else { "Delete" }
                    }
                }
                if let Some(err) = delete_error.read().clone() {
                    div { class: "mt-1 text-xs text-rose-300", "{err}" }
                }
            }
        }
    }
}
