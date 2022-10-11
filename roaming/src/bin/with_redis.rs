use axum::{routing, Json, Router};
use redis::aio::Connection;
use redis::{AsyncCommands, Client};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

const REDIS_DSN: &'static str = "redis://localhost:6379";

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/set", routing::get(set))
        .route("/get", routing::get(get))
        .route("/set_user", routing::get(set_user))
        .route("/get_user", routing::get(get_user));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_connection() -> Result<Connection, String> {
    let client = Client::open(REDIS_DSN).map_err(|e| e.to_string())?;
    client
        .get_async_connection()
        .await
        .map_err(|e| e.to_string())
}

async fn set() -> Result<&'static str, String> {
    let mut conn = get_connection().await?;
    conn.set("author", "Rex Wang")
        .await
        .map_err(|e| e.to_string())?;
    Ok("Successfully set!")
}

async fn get() -> Result<String, String> {
    // 由于 redis 保存的是底层的数据，所以你可以根据需要将读取到的数据进行类型转换，官方文档有示例。
    let mut conn = get_connection().await?;
    let res = conn.get("author").await.map_err(|e| e.to_string())?;
    Ok(res)
}

#[derive(Serialize, Deserialize)]
struct UserInfo {
    id: i32,
    name: String,
}

async fn set_user() -> Result<&'static str, String> {
    let mut conn = get_connection().await?;
    let usr = UserInfo {
        id: 42,
        name: "Rex Wang".to_string(),
    };

    let usr = json!(usr);
    conn.set("user", usr.to_string())
        .await
        .map_err(|e| e.to_string())?;
    Ok("Successfully set user!")
}

async fn get_user() -> Result<Json<UserInfo>, String> {
    let mut conn = get_connection().await?;
    let res: String = conn.get("user").await.map_err(|e| e.to_string())?;
    let user = from_str(&res).map_err(|e| e.to_string())?;

    Ok(Json(user))
}
