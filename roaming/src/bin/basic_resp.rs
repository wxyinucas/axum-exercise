// 返回一个自定义的Response & Error

use axum::http::header::HeaderName;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/str", get(show_str))
        .route("/head_code", get(show_code_header))
        .route("/struct", get(show_struct))
        .route("/result", get(show_result));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn show_str() -> &'static str {
    "Hello, I am Rex Wang"
}

async fn show_code_header() -> (StatusCode, HeaderMap) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("x-powered"),
        HeaderValue::from_static("axum.rs"),
    );

    (StatusCode::OK, headers)
}

async fn show_struct() -> Json<User> {
    let user = User {
        uuid: 0,
        name: "Rex wang".to_string(),
    };

    Json(user)
}

async fn show_result() -> Result<(), MyError>{
    Err(MyError{message: "Something went wrong....".to_string()})
}


#[derive(Serialize)]
struct User {
    uuid: usize,
    name: String,
}

struct MyError {
    message: String,
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        self.message.into_response()
    }
}
