use std::collections::HashMap;

use axum::extract::{Path, Query};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/get_name/:name", get(exact_str))
        .route("/get_user/:name/:id", get(exact_struct))
        .route("/get_pairs", get(exact_pairs))
        .route("/all_query", get(get_all_queries))
        .route("/post_user", post(post_user))
        .route("/json_user", post(json_user))
        .route("/get_all_headers/:name", get(get_all_headers));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn exact_str(Path(name): Path<String>) -> String {
    format!("My name is {}", name)
}

async fn exact_struct(Path(user): Path<User>) -> String {
    format!("GET  name: {}, id: {}", user.name, user.id)
}

#[derive(Deserialize)]
struct User {
    name: String,
    id: usize,
}

// Url <==> Query
async fn exact_pairs(Query(user): Query<User>) -> String {
    format!("QUERY  name: {}, id: {}", user.name, user.id)
}

async fn get_all_queries(Query(args): Query<HashMap<String, String>>) -> String {
    format!("{:?}", args)
}

// Form <==> Form + Post
async fn post_user(Form(user): Form<User>) -> String {
    format!("FORM name: {}, id: {}", user.name, user.id)
}

async fn json_user(Json(user): Json<User>) -> String {
    format!("Json name: {}, id: {}", user.name, user.id)
}

async fn get_all_headers(Path(name): Path<String>, headers: HeaderMap) -> String {
    format!("name:{}\n{:?}", name, headers)
}
