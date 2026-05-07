//! File-based router — inspired by TanStack Router / Nuxt's `pages/` folder.
//!
//! At build time the host crate's `build.rs` walks `resources/routes/**/*.rs`
//! and emits a `routes()` function returning `&[Route]`. At runtime this
//! module provides a tiny dispatcher (`FileRouter`), the reactive
//! `CURRENT_PATH` signal, and helpers for reading route parameters.

use dioxus::prelude::*;
use std::collections::HashMap;

/// A single route discovered from the filesystem.
#[derive(Clone, Copy)]
pub struct Route {
    /// URL pattern — `/users/:id`, `/posts`, `/`, etc.
    pub path: &'static str,
    /// Component factory — returns the rendered element.
    pub component: fn() -> Element,
}

/// Active path. Components subscribe via `use_router_path`; navigation
/// writes it via `navigate`.
pub static CURRENT_PATH: GlobalSignal<String> =
    Signal::global(|| initial_path());

/// Currently matched route pattern. Set by `FileRouter::render` when it
/// dispatches a route. Used by `use_route_params` to extract `:param` values.
pub static MATCHED_PATTERN: GlobalSignal<&'static str> =
    Signal::global(|| "");

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

/// Reactive accessor — returns the current path and re-renders on change.
pub fn use_router_path() -> String {
    CURRENT_PATH.read().clone()
}

/// Read the current path without subscribing.
pub fn current_path() -> String {
    CURRENT_PATH.peek().clone()
}

/// Reactive accessor for the active route's `:param` values.
///
/// Inside `resources/routes/users/[id].rs` you can call
/// `use_route_params().get("id")` to read the matched value.
pub fn use_route_params() -> HashMap<String, String> {
    let path = use_router_path();
    let pattern = *MATCHED_PATTERN.read();
    extract_params(pattern, &path).unwrap_or_default()
}

/// Imperatively navigate. Updates the reactive signal and pushes browser
/// history on web targets.
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

    /// Render the route matching the current path. Routes are matched in
    /// declaration order; longer/more-specific patterns must come first
    /// (the build script handles that automatically).
    pub fn render(self) -> Element {
        let path = use_router_path();
        for r in self.routes {
            if path_matches(r.path, &path) {
                if *MATCHED_PATTERN.peek() != r.path {
                    *MATCHED_PATTERN.write() = r.path;
                }
                return (r.component)();
            }
        }
        (self.not_found)()
    }
}

/// Path matcher — supports literal segments and `:param`.
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
pub fn extract_params(pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let p_segs: Vec<&str> = pattern.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    let a_segs: Vec<&str> = path.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    if p_segs.len() != a_segs.len() {
        return None;
    }
    let mut map = HashMap::new();
    for (p, a) in p_segs.iter().zip(a_segs.iter()) {
        if let Some(name) = p.strip_prefix(':') {
            map.insert(name.to_string(), (*a).to_string());
        } else if p != a {
            return None;
        }
    }
    Some(map)
}
