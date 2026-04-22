//! `/users` — uses the Vue-inspired `ref_` reactive primitive and the
//! shared `User` model from `/models` to render a list.

use dioxus::prelude::*;
use example_app::models::User;
use tars_frontend::prelude::*;

pub fn component() -> Element {
    let users = ref_::<Vec<User>>(vec![]);
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
