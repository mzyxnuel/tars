//! `/users/:id` — show a single user.

use dioxus::prelude::*;
use tars_frontend::prelude::*;
use tars_frontend::Link;
use tars_ui::prelude::*;

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
        Container {
            Page {
                title: "User".to_string(),
                actions: rsx! {
                    Link { to: "/users".to_string(),
                        Button { variant: ButtonVariant::Ghost, "← Back" }
                    }
                    Link { to: edit_path,
                        Button { variant: ButtonVariant::Secondary, "Edit" }
                    }
                },
                if let Some(err) = error.read().clone() {
                    Alert { variant: AlertVariant::Error, "{err}" }
                } else if *loading.read() {
                    div { class: "tars-row", Spinner {} span { class: "tars-muted", "Loading…" } }
                } else if let Some(u) = user.read().clone() {
                    Card { CardBody {
                        div { class: "tars-stack",
                            div {
                                div { class: "tars-label", "ID" }
                                div { "{u.id.unwrap_or(0)}" }
                            }
                            div {
                                div { class: "tars-label", "Name" }
                                div { "{u.name}" }
                            }
                            div {
                                div { class: "tars-label", "Email" }
                                div { "{u.email}" }
                            }
                            if let Some(ts) = u.created_at.as_ref() {
                                div {
                                    div { class: "tars-label", "Created" }
                                    div { class: "tars-muted", "{ts}" }
                                }
                            }
                        }
                    } }
                }
            }
        }
    }
}
