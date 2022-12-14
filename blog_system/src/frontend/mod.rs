use axum::routing::get;
use axum::Router;

use crate::backend::handlers::auth::{login, login_ui, logout};
use crate::frontend::handler::index;

mod handler;
mod view;

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/auth", get(login_ui).post(login))
        .route("/logout", get(logout))
}
