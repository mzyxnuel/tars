use axum::extract::{FromRequest, Request as AxumRequest};
use axum::http::{HeaderMap, Method, Uri};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::error::{Error, Result};

/// Laravel-style request wrapper. Holds JSON body, query params, headers, and
/// route parameters. Cheap to clone (body kept as parsed `serde_json::Value`).
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: HeaderMap,
    pub query: HashMap<String, String>,
    pub body: serde_json::Value,
    pub route_params: HashMap<String, String>,
}

impl Request {
    /// Retrieve an input value from the JSON body or query string — mirrors
    /// Laravel's `$request->input('key')`.
    pub fn input(&self, key: &str) -> Option<serde_json::Value> {
        if let Some(v) = self.body.get(key) {
            return Some(v.clone());
        }
        self.query.get(key).map(|s| serde_json::Value::String(s.clone()))
    }

    /// Same as `input` but with a default fallback.
    pub fn input_or(&self, key: &str, default: serde_json::Value) -> serde_json::Value {
        self.input(key).unwrap_or(default)
    }

    /// Return `true` if the given key exists in body or query.
    pub fn has(&self, key: &str) -> bool {
        self.input(key).is_some()
    }

    /// Return a route parameter by name — like `$request->route('id')`.
    pub fn route(&self, name: &str) -> Option<&str> {
        self.route_params.get(name).map(|s| s.as_str())
    }

    /// Deserialize the full body into a typed struct — like `$request->validated()`
    /// once paired with a FormRequest.
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_value(self.body.clone())
            .map_err(|e| Error::BadRequest(format!("Invalid JSON body: {e}")))
    }

    /// Return only the specified keys from the body.
    pub fn only(&self, keys: &[&str]) -> serde_json::Value {
        let mut out = serde_json::Map::new();
        for key in keys {
            if let Some(v) = self.body.get(*key) {
                out.insert((*key).to_string(), v.clone());
            }
        }
        serde_json::Value::Object(out)
    }

    /// Return all keys except the specified ones.
    pub fn except(&self, keys: &[&str]) -> serde_json::Value {
        let obj = match &self.body {
            serde_json::Value::Object(m) => m.clone(),
            _ => serde_json::Map::new(),
        };
        let mut out = serde_json::Map::new();
        for (k, v) in obj {
            if !keys.contains(&k.as_str()) {
                out.insert(k, v);
            }
        }
        serde_json::Value::Object(out)
    }

    /// Read a header value as a string slice.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|h| h.to_str().ok())
    }

    /// Build a request from an axum request. Consumes the body.
    pub async fn from_axum<S>(req: AxumRequest, state: &S) -> Result<Self>
    where
        S: Send + Sync,
    {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let headers = req.headers().clone();

        let query: HashMap<String, String> = uri
            .query()
            .map(|q| {
                q.split('&')
                    .filter_map(|pair| {
                        let mut it = pair.splitn(2, '=');
                        Some((it.next()?.to_string(), it.next().unwrap_or("").to_string()))
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Try parsing body as JSON. Missing/empty bodies become Value::Null.
        let body = if method == Method::GET || method == Method::DELETE {
            serde_json::Value::Null
        } else {
            match axum::Json::<serde_json::Value>::from_request(req, state).await {
                Ok(axum::Json(v)) => v,
                Err(_) => serde_json::Value::Null,
            }
        };

        Ok(Self {
            method,
            uri,
            headers,
            query,
            body,
            route_params: HashMap::new(),
        })
    }
}
