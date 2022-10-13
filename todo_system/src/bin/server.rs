use axum::{routing, Router};

use todo_system::*;

#[tokio::main]
async fn main() {
    let config = Config::load().unwrap();
    let app = Router::new().route("/", routing::get(usage));

    axum::Server::bind(&(config.web_config.addr).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
