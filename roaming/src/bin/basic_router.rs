use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Form, Router};
use serde::Deserialize;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "basic_router=debug,tower_http=debug");
    }
    tracing_subscriber::fmt::init();

    let news_router = Router::new()
        .route("/", get(news_index))
        .route("/detail/:id", get(news_detail))
        .route("/comments/:id", get(news_comments));

    let app = Router::new()
        .route("/user/:id", get(get_user).post(edit_user))
        .route("/redirect", get(redirect))
        .nest("/news", news_router)
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

struct CheckUser {
    id: i32,
    name: String,
}

#[derive(Deserialize)]
struct EditUser {
    id: i32,
    name: String,
}

async fn get_user(Path(id): Path<i32>) -> Html<String> {
    let model = CheckUser {
        id,
        name: "Rex Wang".to_string(),
    };

    let html = format!(
        r#"<!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/edit_user/{}">
          <input type="hidden" name="id" value="{}">
          <div>
            <label>用户名</label>
            <input type="text" name="username" value="{}">
          </div>
          <div>
            <label>Email</label>
            <input type="email" name="email" value="{}">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>"#,
        model.id, model.id, model.name, model.name
    );
    Html(html)
}

async fn edit_user(Form(form): Form<EditUser>) -> Html<String> {
    // 结合路由，这里的意思是，把id=id的用户给修改了。
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              修改用户-AXUM中文网
            </title>
          </head>
          <body>
            <h1>修改成功！</h1>
            <p>修改后的用户资料：</p>
            <div>ID: {} </div>
            <div>用户名: {} </div>
            <div>Email: {} </div>
          </body>
          </html>"#,
        form.id, form.name, form.name
    );
    Html(html)
}

async fn news_index() -> &'static str {
    "new index"
}

async fn news_detail(Path(id): Path<i32>) -> String {
    format!("news id: {}", id)
}

async fn news_comments(Path(id): Path<i32>) -> String {
    format!("comments id: {}", id)
}

async fn redirect() -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::LOCATION,
        "http://axum.rs".parse().unwrap(),
    );
    (StatusCode::FOUND, headers, ())
}
