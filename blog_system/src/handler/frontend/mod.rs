use axum::{routing, Router};

pub(crate) mod index;

pub fn router() -> Router {
    Router::new().route("/", routing::get(index::index))
}
