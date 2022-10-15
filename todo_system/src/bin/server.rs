use axum::routing::get;
use axum::{routing, Extension, Router};
use sqlx::PgPool;

use todo_system::*;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server=debug,todo_system=debug");
    }
    tracing_subscriber::fmt::init();

    let config = structs::Config::load().unwrap();
    let pool = PgPool::connect(&config.postgres_config.make_address())
        .await
        .map_err(TodoError::from)?;
    let app = Router::new()
        .route("/", routing::get(usage))
        // .route("/todo", get(todo_list::get_all))
        .route("/todo", get(todo_list::get_all).post(todo_list::create))
        .route(
            "/todo/:list_id",
            get(todo_list::find)
                .put(todo_list::update) // todo 这个函数的参数设计的太nm不合理了，这和路由有毛线关系
                .delete(todo_list::delete),
        )
        .route(
            "/todo/:list_id/items",
            get(todo_item::get_all).post(todo_item::create),
        )
        .route(
            "/todo/:list_id/items/:item_id",
            get(todo_item::find)
                .put(todo_item::check)
                .delete(todo_item::delete),
        )
        .layer(Extension(pool));

    let addr = &(config.web_config.addr);
    tracing::info!("Listening on: {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
