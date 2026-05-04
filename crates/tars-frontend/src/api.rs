//! JSON API client. On the web target it uses `gloo-net::http`. Other
//! targets get a placeholder so the library still compiles — desktop and
//! mobile transports can be added as needed.

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

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url.trim_end_matches('/'), path)
    }

    /// GET a JSON resource.
    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let url = self.url(path);
            let resp = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|e| e.to_string())?;
            resp.json::<T>().await.map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = path;
            Err("API transport not configured for this target".into())
        }
    }

    /// POST a JSON body and decode the JSON response into `T`.
    pub async fn post<B: Serialize, T: DeserializeOwned>(&self, path: &str, body: &B) -> Result<T, String> {
        #[cfg(target_arch = "wasm32")]
        {
            let url = self.url(path);
            let resp = gloo_net::http::Request::post(&url)
                .json(body)
                .map_err(|e| e.to_string())?
                .send()
                .await
                .map_err(|e| e.to_string())?;
            resp.json::<T>().await.map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (path, body);
            Err("API transport not configured for this target".into())
        }
    }

    /// POST a JSON body and return the raw JSON response.
    pub async fn post_json(&self, path: &str, body: Value) -> Result<Value, String> {
        self.post::<Value, Value>(path, &body).await
    }
}
