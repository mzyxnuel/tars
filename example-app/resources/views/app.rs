//! Reference root component — equivalent of Laravel's
//! `resources/views/app.blade.php`. The actual frontend binary lives in
//! `frontend/src/main.rs`; this file is kept as a documented example of how
//! a hand-written `App` component would compose the file router.
//!
//! This file is intentionally NOT compiled by the frontend binary (the
//! binary uses the build-time generated route table instead).

#![allow(dead_code)]

use dioxus::prelude::*;
use tars_frontend::prelude::*;

pub fn App() -> Element {
    let router = FileRouter::new(&[], not_found);
    rsx! {
        div { { router.render() } }
    }
}

fn not_found() -> Element {
    rsx! { div { "404" } }
}
