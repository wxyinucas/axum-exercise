use axum::routing::get;
use axum::Router;

mod handlers;
mod view;
use handlers::category;
use crate::backend::handlers::index::index;

pub fn router() -> Router {
    let category_router = Router::new()
        .route("/", get(category::index))
        .route("/add", get(category::add_ui).post(category::add))
        .route("/del/:id", get(category::del))
        .route("/edit/:id", get(category::edit_ui).post(category::edit));
    Router::new()
        .route("/", get(index))
        .nest("/category", category_router)
}
