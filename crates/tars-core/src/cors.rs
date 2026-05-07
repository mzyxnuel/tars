//! CORS middleware. Adds permissive `Access-Control-*` headers and short
//! circuits OPTIONS preflights so the frontend can hit the backend during
//! development. Tighten the allowed origins list before going to prod.

use async_trait::async_trait;
use axum::http::{HeaderName, HeaderValue, StatusCode};

use crate::error::Result;
use crate::middleware::{Middleware, Next};
use crate::request::Request;
use crate::response::Response;

#[derive(Clone)]
pub struct Cors {
    pub allow_origin: String,
    pub allow_headers: String,
    pub allow_methods: String,
    pub allow_credentials: bool,
}

impl Cors {
    /// Permissive defaults — fine for development, tighten for prod.
    pub fn permissive() -> Self {
        Self {
            allow_origin: "*".to_string(),
            allow_headers: "Content-Type, Authorization, Accept".to_string(),
            allow_methods: "GET, POST, PUT, PATCH, DELETE, OPTIONS".to_string(),
            allow_credentials: false,
        }
    }

    pub fn allow_origin(mut self, origin: impl Into<String>) -> Self {
        self.allow_origin = origin.into();
        self
    }

    pub fn allow_credentials(mut self, on: bool) -> Self {
        self.allow_credentials = on;
        self
    }
}

#[async_trait]
impl Middleware for Cors {
    async fn handle(&self, req: Request, next: Next) -> Result<Response> {
        // OPTIONS preflight — return 204 with the right headers.
        if req.method == axum::http::Method::OPTIONS {
            let mut resp = Response::no_content();
            self.apply_headers(&mut resp);
            return Ok(resp);
        }
        let mut resp = next.run(req).await?;
        self.apply_headers(&mut resp);
        Ok(resp)
    }
}

impl Cors {
    fn apply_headers(&self, resp: &mut Response) {
        let pairs = [
            ("Access-Control-Allow-Origin", self.allow_origin.as_str()),
            ("Access-Control-Allow-Headers", self.allow_headers.as_str()),
            ("Access-Control-Allow-Methods", self.allow_methods.as_str()),
        ];
        for (k, v) in pairs {
            if let (Ok(name), Ok(val)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(v)) {
                resp.headers.insert(name, val);
            }
        }
        if self.allow_credentials {
            if let Ok(name) = HeaderName::from_bytes(b"Access-Control-Allow-Credentials") {
                resp.headers.insert(name, HeaderValue::from_static("true"));
            }
        }
        // Make sure the response carries an OK status when none was set.
        if resp.status == StatusCode::default() {
            resp.status = StatusCode::OK;
        }
    }
}
