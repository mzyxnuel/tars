//! `/users` — index. Lists every user in a table with view / edit / delete
//! actions and a button to open the create form.

use dioxus::prelude::*;
use tars_frontend::prelude::*;
use tars_frontend::Link;
use tars_ui::prelude::*;

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
        Container {
            Page {
                title: format!("Users ({count})"),
                actions: rsx! {
                    Link { to: "/users/create".to_string(),
                        Button { variant: ButtonVariant::Primary, "Create user" }
                    }
                },

                if let Some(e) = error.read().clone() {
                    Alert { variant: AlertVariant::Error, "{e}" }
                }
                if *loading.read() {
                    div { class: "tars-row", Spinner {} span { class: "tars-muted", "Loading…" } }
                } else if users.read().is_empty() {
                    Card { CardBody {
                        p { class: "tars-muted", "No users yet. Click \"Create user\" to add one." }
                    } }
                } else {
                    Card {
                        Table {
                            Thead {
                                Th { "ID" } Th { "Name" } Th { "Email" } Th { "" }
                            }
                            tbody {
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

    rsx! {
        tr {
            td { "{id_str}" }
            td { "{user.name}" }
            td { "{user.email}" }
            td {
                div { class: "tars-row",
                    Link { to: format!("/users/{}", id_str),
                        Button { variant: ButtonVariant::Ghost, "View" }
                    }
                    Link { to: format!("/users/{}/edit", id_str),
                        Button { variant: ButtonVariant::Secondary, "Edit" }
                    }
                    Button {
                        variant: ButtonVariant::Danger,
                        disabled: *deleting.read(),
                        onclick: on_delete,
                        if *deleting.read() { "Deleting…" } else { "Delete" }
                    }
                }
                if let Some(err) = delete_error.read().clone() {
                    div { class: "tars-error", "{err}" }
                }
            }
        }
    }
}
