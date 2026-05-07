//! Web routes. Backend is JSON-only (`/api/*`). The root `/` and any other
//! non-API path falls through to the static SPA bundle in `public/`, which
//! is the Dioxus frontend built with `dx build --features web`. Add custom
//! server-rendered HTML routes here if you want backend-rendered pages
//! alongside the SPA.
use tars_core::Router;

pub fn register(_router: &mut Router) {
    // Intentionally empty — see `bootstrap/server.rs` where the SPA bundle
    // is mounted via `Application::with_public_dir(...)`.
}
