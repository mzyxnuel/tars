//! Root view — equivalent to Laravel's `resources/views/app.blade.php`.
//! Hosts the `FileRouter` that dispatches to the generated route tree.

use dioxus::prelude::*;
use tars_frontend::prelude::*;

use crate::resources::routes;

/// The generated route table. In a real build step the CLI walks
/// `resources/routes/**/*.rs` and emits this list automatically; for the
/// MVP it's hand-wired.
pub const ROUTES: &[Route] = &[
    Route { path: "/", component: routes::index::component },
    Route { path: "/users", component: routes::users::component },
];

pub fn App() -> Element {
    let router = FileRouter::new(ROUTES, not_found);
    // For MVP we use a fixed path; real impl would read from Dioxus router.
    router.render("/")
}

fn not_found() -> Element {
    rsx! { div { "404" } }
}
