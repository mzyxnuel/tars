//! `/users/create` — Tailwind-styled form to add a new user. Posts JSON to
//! /api/users; 422 validation errors are rendered field-by-field.

use dioxus::prelude::*;
use serde_json::json;
use tars_frontend::prelude::*;
use tars_frontend::Link;

use crate::shared::{ApiError, ItemResponse, User};

pub fn component() -> Element {
    let mut name = use_field("");
    let mut email = use_field("");
    let mut errors = use_validation_errors();
    let mut submit_error = use_signal::<Option<String>>(|| None);
    let mut submitting = use_signal(|| false);

    let on_submit = move |_evt: MouseEvent| {
        let payload = json!({ "name": name.get(), "email": email.get() });
        spawn(async move {
            submitting.set(true);
            submit_error.set(None);
            errors.clear();

            let api = Api::default_base();
            match api.post::<serde_json::Value, ItemResponse<User>>("/users", &payload).await {
                Ok(_) => navigate("/users"),
                Err(raw) => {
                    if let Some(rest) = raw.strip_prefix("HTTP 422: ") {
                        if let Ok(body) = serde_json::from_str::<ApiError>(rest) {
                            errors.set(body.errors);
                            submit_error.set(Some(body.message));
                        } else {
                            submit_error.set(Some(raw));
                        }
                    } else {
                        submit_error.set(Some(raw));
                    }
                }
            }
            submitting.set(false);
        });
    };

    let name_err = errors.first("name");
    let email_err = errors.first("email");

    rsx! {
        section { class: "max-w-xl mx-auto",
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold tracking-tight", "Create user" }
                Link { class: "text-sm text-slate-400 hover:text-slate-200".to_string(),
                    to: "/users".to_string(), "← Back"
                }
            }

            if let Some(err) = submit_error.read().clone() {
                div { class: "mb-4 px-4 py-3 rounded-md border border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm",
                    "{err}"
                }
            }

            form { class: "space-y-5 p-6 rounded-xl border border-slate-800 bg-slate-900/40",
                Field {
                    label: "Name",
                    placeholder: "Ada Lovelace",
                    value: name.get(),
                    error: name_err,
                    on_input: EventHandler::new(move |v: String| name.set(v)),
                }
                Field {
                    label: "Email",
                    input_type: "email",
                    placeholder: "ada@example.com",
                    value: email.get(),
                    error: email_err,
                    on_input: EventHandler::new(move |v: String| email.set(v)),
                }
                div { class: "flex gap-3 pt-2",
                    button {
                        r#type: "button",
                        class: "px-5 py-2 rounded-md bg-indigo-500 hover:bg-indigo-400 text-white font-medium text-sm shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition",
                        disabled: *submitting.read(),
                        onclick: on_submit,
                        if *submitting.read() { "Saving…" } else { "Create user" }
                    }
                    Link { class: "px-5 py-2 rounded-md border border-slate-700 hover:border-slate-500 text-slate-200 font-medium text-sm transition".to_string(),
                        to: "/users".to_string(), "Cancel"
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct FieldProps {
    label: &'static str,
    #[props(default = "text")]
    input_type: &'static str,
    #[props(default = "")]
    placeholder: &'static str,
    value: String,
    error: Option<String>,
    on_input: EventHandler<String>,
}

#[component]
fn Field(props: FieldProps) -> Element {
    let border = if props.error.is_some() {
        "border-rose-500/60 focus:border-rose-400 focus:ring-rose-400/30"
    } else {
        "border-slate-700 focus:border-indigo-400 focus:ring-indigo-400/30"
    };
    rsx! {
        label { class: "block",
            span { class: "block text-xs font-medium text-slate-400 uppercase tracking-wider mb-1.5",
                "{props.label}"
            }
            input {
                r#type: "{props.input_type}",
                placeholder: "{props.placeholder}",
                value: "{props.value}",
                class: "w-full px-3 py-2 rounded-md bg-slate-950/60 text-slate-100 placeholder-slate-500 border {border} focus:outline-none focus:ring-4 transition",
                oninput: move |evt| props.on_input.call(evt.value()),
            }
            if let Some(err) = props.error.clone() {
                span { class: "block mt-1.5 text-xs text-rose-300", "{err}" }
            }
        }
    }
}
