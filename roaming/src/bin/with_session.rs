use askama::Template;
use axum::extract::Query;
use axum::headers::{Cookie, HeaderMapExt};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Html;
use axum::{routing, Extension, Form, Router};
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let rdc = redis::Client::open(REDIS_DSN).unwrap();
    let app = Router::new()
        .route("/", routing::get(index_page))
        .route("/login", routing::get(login_page).post(login_action)) // todo 注意html的form标签
        .route("/logout", routing::get(logout_action))
        .layer(Extension(rdc));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const SESSION_ID_COOKIE_NAME: &str = "axum_rs_session_id";
const SESSION_KEY_PREFIX: &str = "axum_rs_session";
const REDIS_DSN: &str = "redis://127.0.0.1:6379/";

#[derive(Serialize, Deserialize, Debug)]
struct UserSession {
    // 被保存的信息
    username: String,
    level: u8,
}

#[derive(Deserialize)]
struct UserLoginForm {
    // 从html里传递的信息，todo 如何传递的？
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginMessage {
    // to-do 现在有bug，如何与template更好地结合？
    msg: Option<String>,
}

#[derive(Template)]
#[template(path = "login_page.html")]
struct LoginPage {
    message: String,
}

fn save_session_to_cookie(session_id: &str, headers: &mut HeaderMap) {
    // todo cookie 与 session 的关系是？
    let cookie = format!("{}={}", SESSION_ID_COOKIE_NAME, session_id);
    headers.insert(
        axum::http::header::SET_COOKIE,
        cookie.as_str().parse().unwrap(),
    );
}

fn get_session_from_cookie(headers: &HeaderMap) -> Option<String> {
    let cookies = headers.typed_get::<Cookie>(); // todo: 注意如何将cookie取出来，看文档了解headers
    if cookies.is_none() {
        return None;
    }

    let mut session_id = None;
    for (key, value) in cookies.unwrap().iter() {
        if key == SESSION_ID_COOKIE_NAME && value.len() != 0 {
            session_id = Some(value.to_string());
        }
    }

    session_id
}

async fn login_page(Query(login_msg): Query<LoginMessage>) -> Html<String> {
    let msg = match login_msg.msg {
        None => "".to_string(),
        Some(msg) => format!(r#"<div style="color:red">{}</div>"#, msg),
    };

    let page = LoginPage { message: msg };
    Html(page.render().unwrap())
}

async fn login_action(
    Extension(rdc): Extension<redis::Client>,
    Form(frm): Form<UserLoginForm>,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let mut headers = HeaderMap::new();
    let url; // todo！！！ 如何重定向？？？

    if !(&frm.username == "rex_wang" && &frm.password == "axum.rs") {
        url = "/login?msg=用户名或密码错误";
    } else {
        let session_id = uuid::Uuid::new_v4().as_simple().to_string();
        save_session_to_cookie(&session_id, &mut headers);

        let user_session = UserSession {
            username: frm.username,
            level: 1,
        };
        let user_session = serde_json::json!(user_session).to_string();

        // redis
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|e| e.to_string())?;
        conn.set_ex(redis_key, user_session, 1200)
            .await
            .map_err(|e| e.to_string())?;
        url = "/";
    }

    headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}

async fn logout_action(
    Extension(rdc): Extension<redis::Client>,
    headers: HeaderMap,
) -> Result<(StatusCode, HeaderMap, ()), String> {
    let session_id = get_session_from_cookie(&headers);
    let mut headers = HeaderMap::new();
    if let Some(session_id) = session_id {
        // delete session from redis
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|err| err.to_string())?;
        conn.del(redis_key).await.map_err(|err| err.to_string())?;

        // 清空cookies
        save_session_to_cookie(&session_id, &mut headers);
    }

    headers.insert(axum::http::header::LOCATION, "/login".parse().unwrap());
    Ok((StatusCode::FOUND, headers, ()))
}

async fn index_page(
    Extension(rdc): Extension<redis::Client>,
    headers: HeaderMap,
) -> Result<Html<String>, String> {
    let session_id = get_session_from_cookie(&headers);
    let mut session = None;
    if let Some(session_id) = session_id {
        let redis_key = format!("{}{}", SESSION_KEY_PREFIX, session_id);
        let mut conn = rdc
            .get_async_connection()
            .await
            .map_err(|err| err.to_string())?;
        let session_str: Option<String> =
            conn.get(redis_key).await.map_err(|err| err.to_string())?;

        if let Some(session_str) = session_str {
            let user_session: UserSession =
                serde_json::from_str(&session_str).map_err(|e| e.to_string())?;
            session = Some(user_session);
        }
    }

    match session {
        Some(session) => {
            let html = format!(
                r#"
        <!DOCTYPE html>
        <html lang="zh-Hans">
          <head>
            <meta charset="utf-8" />
            <meta name="author" content="axum.rs (team@axum.rs)" />
            <title>
              用户首页-AXUM中文网
            </title>
          </head>
          <body>
            <div>欢迎 {} ! 你的等级是 {}。</div>
            <div><a href="/logout">退出登录</a></div>
          </body>
          </html>"#,
                session.username, session.level
            );
            Ok(Html(html))
        }
        None => Err("Please login via /login page".to_string()),
    }
}
