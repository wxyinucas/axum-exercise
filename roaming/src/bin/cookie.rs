use axum::headers::Cookie;
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::routing::get;
use axum::{Form, Router, TypedHeader};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        // TODO: 展示的逻辑和操作的逻辑
        .route("/", get(user_center))
        .route("/login", get(user_login).post(login_action))
        .route("/logout", get(user_logout));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const COOKIE_NAME: &'static str = "name";

#[derive(Deserialize)]
struct LoginUserForm {
    name: String,
    password: String,
}

async fn login_action(Form(form): Form<LoginUserForm>) -> (StatusCode, HeaderMap, ()) {
    let mut headers = HeaderMap::new();

    if !(&form.name == "Rex Wang" && &form.password == "axum.rs") {
        headers.insert(
            axum::http::header::LOCATION,
            "/login?msg=用户名或密码错误".parse().unwrap(),
        );
    } else {
        let cookie = format!("{}={}", COOKIE_NAME, form.name);
        headers.insert(
            axum::http::header::SET_COOKIE,
            cookie.as_str().parse().unwrap(),
        ); // 设置Cookie
        headers.insert(axum::http::header::LOCATION, "/".parse().unwrap()); // 跳转到用户中心首页
    }
    (StatusCode::FOUND, headers, ())
}

async fn user_center(
    TypedHeader(cookies): TypedHeader<Cookie>,
) -> Result<Html<String>, &'static str> {
    if cookies.len() == 0 {
        return Err("NO COOKIES SET!");
    };

    let mut login_name: Option<String> = None;
    for (key, value) in cookies.iter() {
        if key == COOKIE_NAME && value != "" {
            login_name = Some(value.to_owned());
            break;
        }
    }

    if login_name.is_none() {
        return Err("COOKIE IS USELESS!");
    }

    let html = format!(
        r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户中心-AXUM中文网
            </title>
          </head>
          <body>
          <p>你好，<strong>{}</strong>！你已成功登录。[<a href="/logout">退出登录</a>]
          </body>
          </html>
        "#,
        login_name.unwrap()
    );
    Ok(Html(html))
}

async fn user_logout() -> (StatusCode, HeaderMap, ()) {
    let cookie = format!("{}=", COOKIE_NAME);
    let mut headers = HeaderMap::new();
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    ); // 清空Cookie
    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap()); // 跳转到登录页面
    (StatusCode::FOUND, headers, ()) // TODO 重定向
}

async fn user_login() -> Html<String> {
    let html = r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户登录-AXUM中文网
            </title>
          </head>
          <body>
          <form method="post" action="/login">
          <div>
            <label>用户名</label>
            <input type="text" name="username">
          </div>
          <div>
            <label>密码</label>
            <input type="password" name="password">
          </div>
          <div>
            <button type="submit">提交</button>
          </div>
          </form>
          </body>
          </html>
        "#
    .to_string();

    Html(html)
}
