use axum::routing::get;
use axum::{routing, Extension, Router};
use sqlx::PgPool;

use todo_system::structs::TodoList;
use todo_system::*;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server=debug");
    }
    tracing_subscriber::fmt::init();

    let config = structs::Config::load().unwrap();
    let pool = PgPool::connect(&config.postgres_config.make_address())
        .await
        .map_err(|e| <sqlx::Error as Into<TodoError>>::into(e))?;
    let app = Router::new()
        // .route("/", routing::get(usage))
        .route("/todo", get(TodoList::get_all).post(TodoList::create))
        // .route(
        //     "/todo/:list_id",
        //     get(TodoList::find)
        //         .put(TodoList::update)
        //         .delete(TodoList::delete),
        // )
        .layer(Extension(pool));

    let addr = &(config.web_config.addr);
    tracing::info!("Listening on: {}", addr);
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
