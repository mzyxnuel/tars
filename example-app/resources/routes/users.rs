//! `/users` — uses the Vue-inspired `ref_` reactive primitive plus the
//! `Api` JSON client to fetch the same endpoint the backend exposes.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tars_frontend::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct User {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ListResponse {
    pub data: Vec<User>,
}

pub fn component() -> Element {
    let users = ref_::<Vec<User>>(vec![]);
    let mut users_clone = users;

    use_future(move || async move {
        let api = Api::new("/api");
        if let Ok(resp) = api.get::<ListResponse>("/users").await {
            users_clone.set(resp.data);
        }
    });

    rsx! {
        div {
            h1 { "Users" }
            ul {
                for u in users.read().iter() {
                    li { key: "{u.id:?}", "{u.name} — {u.email}" }
                }
            }
        }
    }
}
