//! `/users/create` — form to add a new user. Posts JSON to /api/users,
//! handles 422 validation errors by re-rendering field-level messages,
//! navigates to /users on success.

use dioxus::prelude::*;
use serde_json::json;
use tars_frontend::prelude::*;
use tars_frontend::Link;
use tars_ui::prelude::*;

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
        Container {
            Page {
                title: "Create user".to_string(),
                actions: rsx! {
                    Link { to: "/users".to_string(),
                        Button { variant: ButtonVariant::Ghost, "← Back" }
                    }
                },
                if let Some(err) = submit_error.read().clone() {
                    Alert { variant: AlertVariant::Error, "{err}" }
                }
                Card { CardBody {
                    FormGroup {
                        FormField { label: "Name".to_string(), error: name_err,
                            Input {
                                value: name.get(),
                                placeholder: "Ada Lovelace".to_string(),
                                oninput: move |v| name.set(v),
                            }
                        }
                        FormField { label: "Email".to_string(), error: email_err,
                            Input {
                                r#type: "email".to_string(),
                                value: email.get(),
                                placeholder: "ada@example.com".to_string(),
                                oninput: move |v| email.set(v),
                            }
                        }
                        div { class: "tars-row", style: "margin-top: 4px;",
                            Button {
                                variant: ButtonVariant::Primary,
                                disabled: *submitting.read(),
                                onclick: on_submit,
                                if *submitting.read() { "Saving…" } else { "Create user" }
                            }
                            Link { to: "/users".to_string(),
                                Button { variant: ButtonVariant::Ghost, "Cancel" }
                            }
                        }
                    }
                } }
            }
        }
    }
}
