//! `/users/:id/edit` — edit form. Loads the user, lets you change name &
//! email, PUTs the result and navigates back to the show page.

use dioxus::prelude::*;
use serde_json::json;
use tars_frontend::prelude::*;
use tars_frontend::Link;
use tars_ui::prelude::*;

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
        Container {
            Page {
                title: format!("Edit user #{id}"),
                actions: rsx! {
                    Link { to: back_path,
                        Button { variant: ButtonVariant::Ghost, "← Back" }
                    }
                },
                if let Some(err) = load_error.read().clone() {
                    Alert { variant: AlertVariant::Error, "{err}" }
                } else if *loading.read() {
                    div { class: "tars-row", Spinner {} span { class: "tars-muted", "Loading user…" } }
                } else {
                    {
                        let mut name_ref = name;
                        let mut email_ref = email;
                        rsx! {
                            if let Some(err) = submit_error.read().clone() {
                                Alert { variant: AlertVariant::Error, "{err}" }
                            }
                            Card { CardBody {
                                FormGroup {
                                    FormField { label: "Name".to_string(), error: name_err,
                                        Input { value: name_ref.get(), oninput: move |v| name_ref.set(v) }
                                    }
                                    FormField { label: "Email".to_string(), error: email_err,
                                        Input {
                                            r#type: "email".to_string(),
                                            value: email_ref.get(),
                                            oninput: move |v| email_ref.set(v),
                                        }
                                    }
                                    div { class: "tars-row", style: "margin-top: 4px;",
                                        Button {
                                            variant: ButtonVariant::Primary,
                                            disabled: *submitting.read(),
                                            onclick: on_save,
                                            if *submitting.read() { "Saving…" } else { "Save changes" }
                                        }
                                    }
                                }
                            } }
                        }
                    }
                }
            }
        }
    }
}
