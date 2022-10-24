use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BlogError {
    #[error("Sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
    #[error("Config error: {0}")]
    TomlError(#[from] toml::de::Error),
    #[error("Template error: {0}")]
    AskamaError(#[from] askama::Error),
    #[error("BcryptError error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Not found error: {0}")]
    NotFoundError(String),
    #[error("Duplicate error: {0}")]
    DuplicateError(String),

    #[error("Wrong email or password error")]
    IncorrectLoginError,
    #[error("Forbidden!")]
    ForbiddenError,
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        tracing::error!("{}", msg);

        match self {
            BlogError::ForbiddenError => {
                let mut headers = HeaderMap::new();
                headers.insert(header::LOCATION, "/auth".parse().unwrap());
                (StatusCode::FOUND, headers, ()).into_response()
            }
            _ => msg.into_response(),
        }
    }
}
