use axum::http::StatusCode;
use serde_json::json;
use thiserror::Error;

/// Framework-wide error type — returned from controllers, middleware and
/// request lifecycle handlers. Converts into a JSON response automatically.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Not Found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Validation failed")]
    Validation(serde_json::Value),

    #[error("Bad Request: {0}")]
    BadRequest(String),

    #[error("{0}")]
    Http(StatusCode, String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    pub fn status(&self) -> StatusCode {
        match self {
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::Http(s, _) => *s,
            Error::Internal(_) | Error::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Error::Validation(v) => json!({ "message": "The given data was invalid.", "errors": v }),
            Error::Http(_, msg) => json!({ "message": msg }),
            e => json!({ "message": e.to_string() }),
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status = self.status();
        let body = axum::Json(self.to_json());
        (status, body).into_response()
    }
}
