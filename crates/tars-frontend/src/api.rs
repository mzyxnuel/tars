//! JSON API client — lets components talk to the TARS backend using the
//! same data contract as the rest of the framework. For the MVP we only
//! expose a minimal async fetch helper using the browser's `fetch` (web) or
//! `reqwest`-style desktop implementations in future work.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Lightweight JSON API wrapper. Holds a base URL and default headers.
#[derive(Clone, Debug)]
pub struct Api {
    pub base_url: String,
}

impl Api {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// GET a resource and decode JSON. Stub implementation — Dioxus desktop
    /// targets can plug in reqwest, web targets can plug in `gloo-net`.
    pub async fn get<T: DeserializeOwned>(&self, _path: &str) -> Result<T, String> {
        // Placeholder — real impl would pick the right transport per target.
        Err("API transport not yet configured for this target".into())
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, _path: &str, _body: &B) -> Result<T, String> {
        Err("API transport not yet configured for this target".into())
    }

    pub async fn post_json(&self, _path: &str, _body: Value) -> Result<Value, String> {
        Err("API transport not yet configured for this target".into())
    }
}
