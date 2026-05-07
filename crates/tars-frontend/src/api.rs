//! JSON API client. Uses `gloo-net` on wasm32 (web) and a stub elsewhere
//! to keep the lib compiling for desktop/mobile until those transports
//! are wired up. The contract — JSON in, JSON out — matches the backend.

use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

/// Lightweight JSON API wrapper. Holds the API base URL.
#[derive(Clone, Debug)]
pub struct Api {
    pub base_url: String,
}

impl Api {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self { base_url: base_url.into() }
    }

    /// On web, defaults to `/api` (same-origin). On other targets, uses
    /// `http://localhost:8000/api` to mirror the bundled `bootstrap/server.rs`.
    pub fn default_base() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self::new("/api")
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Self::new("http://localhost:8000/api")
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        self.request::<(), T>("GET", path, None).await
    }

    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, String> {
        self.request::<B, T>("POST", path, Some(body)).await
    }

    pub async fn put<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, String> {
        self.request::<B, T>("PUT", path, Some(body)).await
    }

    pub async fn delete(&self, path: &str) -> Result<(), String> {
        self.request::<(), Value>("DELETE", path, None).await.map(|_| ())
    }

    pub async fn post_json(&self, path: &str, body: Value) -> Result<Value, String> {
        self.post::<Value, Value>(path, &body).await
    }

    #[allow(unused_variables)]
    async fn request<B: Serialize, T: DeserializeOwned>(
        &self,
        method: &str,
        path: &str,
        body: Option<&B>,
    ) -> Result<T, String> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_net::http::{Method, Request};
            let url = self.url(path);
            let m = match method {
                "GET" => Method::GET,
                "POST" => Method::POST,
                "PUT" => Method::PUT,
                "PATCH" => Method::PATCH,
                "DELETE" => Method::DELETE,
                _ => Method::GET,
            };
            let mut builder = Request::new(&url).method(m);
            let resp = if let Some(b) = body {
                builder = builder.header("Content-Type", "application/json");
                let json = serde_json::to_string(b).map_err(|e| e.to_string())?;
                builder.body(json).map_err(|e| e.to_string())?
                    .send().await.map_err(|e| e.to_string())?
            } else {
                builder.send().await.map_err(|e| e.to_string())?
            };
            if !(200..300).contains(&resp.status()) {
                let txt = resp.text().await.unwrap_or_default();
                return Err(format!("HTTP {}: {}", resp.status(), txt));
            }
            // 204 → return Null and let serde figure it out for unit/Value.
            if resp.status() == 204 {
                let null = serde_json::Value::Null;
                return serde_json::from_value::<T>(null).map_err(|e| e.to_string());
            }
            resp.json::<T>().await.map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (method, path, body);
            Err("API transport not configured for this target — enable the `web` feature".into())
        }
    }
}
