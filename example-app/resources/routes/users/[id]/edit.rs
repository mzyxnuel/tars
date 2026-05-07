//! `/users/:id/edit` — Tailwind-styled edit form. Loads the user, lets
//! you change name & email, PUTs the result and navigates back.

use dioxus::prelude::*;
use serde_json::json;
use tars_frontend::prelude::*;
use tars_frontend::Link;

use crate::shared::{ApiError, ItemResponse, User};

pub fn component() -> Element {
    let params = use_route_params();
    let id = params.get("id").cloned().unwrap_or_default();

    let mut name = use_field("");
    let mut email = use_field("");
    let mut errors = use_validation_errors();
    let mut load_error = use_signal::<Option<String>>(|| None);
    let mut submit_error = use_signal::<Option<String>>(|| None);
    let mut loading = use_signal(|| true);
    let mut submitting = use_signal(|| false);

    let id_for_fetch = id.clone();
    use_effect(move || {
        let id = id_for_fetch.clone();
        spawn(async move {
            loading.set(true);
            match Api::default_base()
                .get::<ItemResponse<User>>(&format!("/users/{id}"))
                .await
            {
                Ok(resp) => {
                    name.set(resp.data.name);
                    email.set(resp.data.email);
                }
                Err(e) => load_error.set(Some(e)),
            }
            loading.set(false);
        });
    });

    let id_for_save = id.clone();
    let on_save = move |_evt: MouseEvent| {
        let id = id_for_save.clone();
        let payload = json!({ "name": name.get(), "email": email.get() });
        spawn(async move {
            submitting.set(true);
            submit_error.set(None);
            errors.clear();
            let api = Api::default_base();
            match api
                .put::<serde_json::Value, ItemResponse<User>>(&format!("/users/{id}"), &payload)
                .await
            {
                Ok(_) => navigate(&format!("/users/{id}")),
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
    let back_path = format!("/users/{id}");

    rsx! {
        section { class: "max-w-xl mx-auto",
            div { class: "flex items-center justify-between mb-6",
                h1 { class: "text-2xl font-bold tracking-tight", "Edit user #{id}" }
                Link { class: "text-sm text-slate-400 hover:text-slate-200".to_string(),
                    to: back_path, "← Back"
                }
            }

            if let Some(err) = load_error.read().clone() {
                div { class: "px-4 py-3 rounded-md border border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm",
                    "{err}"
                }
            } else if *loading.read() {
                div { class: "flex items-center gap-2 text-slate-400 text-sm",
                    span { class: "inline-block w-4 h-4 border-2 border-slate-700 border-t-indigo-400 rounded-full animate-spin" }
                    span { "Loading user…" }
                }
            } else {
                {
                    let mut name_ref = name;
                    let mut email_ref = email;
                    rsx! {
                        if let Some(err) = submit_error.read().clone() {
                            div { class: "mb-4 px-4 py-3 rounded-md border border-rose-500/40 bg-rose-500/10 text-rose-200 text-sm",
                                "{err}"
                            }
                        }
                        form { class: "space-y-5 p-6 rounded-xl border border-slate-800 bg-slate-900/40",
                            EditField {
                                label: "Name",
                                value: name_ref.get(),
                                error: name_err,
                                on_input: EventHandler::new(move |v: String| name_ref.set(v)),
                            }
                            EditField {
                                label: "Email",
                                input_type: "email",
                                value: email_ref.get(),
                                error: email_err,
                                on_input: EventHandler::new(move |v: String| email_ref.set(v)),
                            }
                            div { class: "pt-2",
                                button {
                                    r#type: "button",
                                    class: "px-5 py-2 rounded-md bg-indigo-500 hover:bg-indigo-400 text-white font-medium text-sm shadow-sm disabled:opacity-50 disabled:cursor-not-allowed transition",
                                    disabled: *submitting.read(),
                                    onclick: on_save,
                                    if *submitting.read() { "Saving…" } else { "Save changes" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct EditFieldProps {
    label: &'static str,
    #[props(default = "text")]
    input_type: &'static str,
    value: String,
    error: Option<String>,
    on_input: EventHandler<String>,
}

#[component]
fn EditField(props: EditFieldProps) -> Element {
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
