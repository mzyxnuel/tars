//! File-based route → `/`. The filename maps to the URL pattern by
//! convention (just like TanStack Router / Nuxt): `index.rs` → `/`,
//! `users.rs` → `/users`, `users/[id].rs` → `/users/:id`.
//!
//! Each file must expose a `component()` function returning a Dioxus Element.

use dioxus::prelude::*;

pub fn component() -> Element {
    rsx! {
        div {
            h1 { "Welcome to TARS" }
            p { "A Laravel-inspired, Rust-native framework." }
        }
    }
}
