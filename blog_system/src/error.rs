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

    #[error("Not found error: {0}")]
    NotFoundError(String),

    #[error("Duplicate error: {0}")]
    DuplicateError(String),
}

impl IntoResponse for BlogError {
    fn into_response(self) -> Response {
        let msg = self.to_string();
        tracing::error!("{}", msg);
        msg.into_response()
    }
}
