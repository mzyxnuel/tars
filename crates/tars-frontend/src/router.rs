//! File-based router — inspired by TanStack Router / Nuxt's `pages/` folder.
//!
//! At build time the host crate's `build.rs` walks `resources/routes/**/*.rs`
//! and emits a `ROUTES` constant of `&[Route]`. At runtime this module
//! provides a tiny dispatcher (`FileRouter`) plus the reactive primitives
//! that components use to read the current path and trigger navigation.

use dioxus::prelude::*;

/// A single route discovered from the filesystem.
#[derive(Clone, Copy)]
pub struct Route {
    /// URL pattern — `/users/:id`, `/posts`, `/`, etc.
    pub path: &'static str,
    /// Component factory — returns the rendered element.
    pub component: fn() -> Element,
}

/// Global signal holding the active path. Components read it through
/// `use_router_path`; navigation writes it via `navigate`.
pub static CURRENT_PATH: GlobalSignal<String> =
    Signal::global(|| initial_path());

fn initial_path() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(path) = window.location().pathname() {
                return path;
            }
        }
    }
    "/".to_string()
}

/// Hook giving components access to the current path. Re-renders on change.
pub fn use_router_path() -> String {
    CURRENT_PATH.read().clone()
}

/// Read the current path without subscribing — useful from non-component code.
pub fn current_path() -> String {
    CURRENT_PATH.peek().clone()
}

/// Imperatively navigate to `path`. Updates the reactive signal so any
/// component reading it re-renders, and pushes browser history on web.
pub fn navigate(path: &str) {
    *CURRENT_PATH.write() = path.to_string();
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(window) = web_sys::window() {
            if let Ok(history) = window.history() {
                let _ = history.push_state_with_url(
                    &wasm_bindgen::JsValue::NULL,
                    "",
                    Some(path),
                );
            }
        }
    }
}

/// File-based router. Holds a list of discovered routes and renders the
/// matching component based on `CURRENT_PATH`.
#[derive(Clone, Copy)]
pub struct FileRouter {
    pub routes: &'static [Route],
    pub not_found: fn() -> Element,
}

impl FileRouter {
    pub const fn new(routes: &'static [Route], not_found: fn() -> Element) -> Self {
        Self { routes, not_found }
    }

    /// Render the router. Reads from `CURRENT_PATH` so the component
    /// re-renders whenever `navigate` is called.
    pub fn render(self) -> Element {
        let path = use_router_path();
        for r in self.routes {
            if path_matches(r.path, &path) {
                return (r.component)();
            }
        }
        (self.not_found)()
    }
}

/// Path matcher — supports literal segments and `:param`. Leaves wildcards
/// and regex-style matches to future work.
pub fn path_matches(pattern: &str, path: &str) -> bool {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let a_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
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

/// Extract `:param` values from a matched path. Returns `None` if the
/// pattern doesn't match.
pub fn extract_params(pattern: &str, path: &str) -> Option<std::collections::HashMap<String, String>> {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let a_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    if p_segs.len() != a_segs.len() {
        return None;
    }
    let mut map = std::collections::HashMap::new();
    for (p, a) in p_segs.iter().zip(a_segs.iter()) {
        if let Some(name) = p.strip_prefix(':') {
            map.insert(name.to_string(), (*a).to_string());
        } else if p != a {
            return None;
        }
    }
    Some(map)
}
