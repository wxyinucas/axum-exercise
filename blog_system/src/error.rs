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

impl BlogError {
    pub fn make_into(error: impl Into<BlogError>) -> Self {
        let res = error.into(); // TODO new 注意From和into？
        tracing::error!("{}", res); // TODO new 这和上面的处理结果完全不一样哦，观察处理的位置是不同的
        res
    }
}

// #[cfg(test)]
