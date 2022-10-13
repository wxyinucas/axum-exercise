use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum TodoError{
    #[error("Postgres Error")]
    PostgresError(#[from] sqlx::Error),

    #[error("Config error")]
    TomlError(#[from] toml::de::Error),
}


impl IntoResponse for TodoError {
    fn into_response(self) -> Response {
        self.to_string().into_response()
    }
}