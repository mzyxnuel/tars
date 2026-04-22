//! File-based router — inspired by TanStack Router / Nuxt's `pages/` folder.
//!
//! Developers place files under `resources/routes/` and TARS builds a Dioxus
//! route tree from them. For the MVP this crate provides the runtime types
//! (`Route`, `FileRouter`) and a `render_routes!` macro-free helper. The CLI
//! does the filesystem scan at build time.

use dioxus::prelude::*;

/// A single route discovered from the filesystem.
#[derive(Clone)]
pub struct Route {
    /// URL pattern — `/users/:id`, `/posts`, `/`, etc.
    pub path: &'static str,
    /// Component factory — returns the rendered element.
    pub component: fn() -> Element,
}

/// File-based router. Holds a list of discovered routes and renders the
/// matching component based on the current location. Relies on Dioxus's
/// own Router/Link primitives for navigation events.
#[derive(Clone)]
pub struct FileRouter {
    pub routes: &'static [Route],
    pub not_found: fn() -> Element,
}

impl FileRouter {
    pub const fn new(routes: &'static [Route], not_found: fn() -> Element) -> Self {
        Self { routes, not_found }
    }

    /// Render the router into the page. Matches based on `window.location`
    /// on web targets and the Dioxus router on desktop/mobile.
    pub fn render(self, current_path: &str) -> Element {
        for r in self.routes {
            if path_matches(r.path, current_path) {
                return (r.component)();
            }
        }
        (self.not_found)()
    }
}

/// Very small path matcher — supports literal segments and `:param`. Leaves
/// advanced matching (wildcards, regex) to future work.
pub fn path_matches(pattern: &str, path: &str) -> bool {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').collect();
    let a_segs: Vec<&str> = path.trim_matches('/').split('/').collect();
    if p_segs.len() != a_segs.len() {
        return false;
    }
    for (p, a) in p_segs.iter().zip(a_segs.iter()) {
        if p.starts_with(':') {
            continue;
        }
        if p != a {
            return false;
        }
    }
    true
}
