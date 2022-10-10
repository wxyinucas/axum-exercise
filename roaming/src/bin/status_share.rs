use std::sync::Arc;
use axum::routing::get;
use axum::{Extension, Router};

#[tokio::main]
async fn main() {
    let user = User {
        uuid: 42,
        name: "Rex Wang".to_string(),
    };

    let app = Router::new()
        .route("/", get(get_user_info))
        .layer(Extension(Arc::new(user)));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct User {
    uuid: usize,
    name: String,
}

async fn get_user_info(Extension(user): Extension<Arc<User>>) -> String {
    format!("FROM EXTENSION, id: {} name: {}", user.uuid, user.name)
}
