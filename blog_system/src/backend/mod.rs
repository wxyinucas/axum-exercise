use axum::routing::get;
use axum::Router;

use handlers::{category, topic};

use crate::backend::handlers::index::index;

pub(crate) mod handlers;
mod view;

pub fn router() -> Router {
    // TODO 如何设计 router 和 mod 的引入？
    let category_router = Router::new()
        .route("/", get(category::index))
        .route("/add", get(category::add_ui).post(category::add))
        .route("/del/:id", get(category::del))
        .route("/edit/:id", get(category::edit_ui).post(category::edit));

    let topic_router = Router::new()
        .route("/", get(topic::index))
        .route("/add", get(topic::add_ui).post(topic::add))
        .route("/edit/:id", get(topic::edit_ui).post(topic::edit))
        .route("/del/:id", get(topic::del));

    Router::new()
        .route("/", get(index))
        .nest("/category", category_router)
        .nest("/topic", topic_router)
}
