use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use serde::Serialize;
use serde_json::Value;

/// Laravel-style response. Always JSON — aligned with the framework's
/// JSON-only transport contract between frontend and backend.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Value,
}

impl Response {
    pub fn new(body: Value) -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        if let (Ok(k), Ok(v)) = (HeaderName::from_bytes(key.as_bytes()), HeaderValue::from_str(value)) {
            self.headers.insert(k, v);
        }
        self
    }

    /// Serialize any `Serialize` value into a JSON 200 response.
    pub fn json<T: Serialize>(value: T) -> Self {
        Self::new(serde_json::to_value(value).unwrap_or(Value::Null))
    }

    /// 204 No Content response.
    pub fn no_content() -> Self {
        Self::new(Value::Null).status(StatusCode::NO_CONTENT)
    }

    /// 201 Created response.
    pub fn created<T: Serialize>(value: T) -> Self {
        Self::json(value).status(StatusCode::CREATED)
    }
}

impl axum::response::IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        let mut resp = (self.status, axum::Json(self.body)).into_response();
        resp.headers_mut().extend(self.headers);
        resp
    }
}

/// Helper trait so controllers can return various values directly.
pub trait IntoResponse {
    fn into_tars_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_tars_response(self) -> Response {
        self
    }
}

impl IntoResponse for Value {
    fn into_tars_response(self) -> Response {
        Response::new(self)
    }
}

impl IntoResponse for () {
    fn into_tars_response(self) -> Response {
        Response::no_content()
    }
}

impl<T: Serialize> IntoResponse for Vec<T> {
    fn into_tars_response(self) -> Response {
        Response::json(self)
    }
}
