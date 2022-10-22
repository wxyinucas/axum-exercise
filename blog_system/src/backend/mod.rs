use axum::Router;
use axum::routing::get;
use crate::backend::handler::index;

mod handler;
mod view;


pub fn router() -> Router {
    Router::new().route("/", get(index))
}