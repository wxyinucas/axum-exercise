// use axum::headers::{Cookie, HeaderMapExt};

use axum::headers::{Cookie, HeaderMapExt};
use axum::http::HeaderMap;

const COOKIE_NAME: &str = "axum_blog_admin";

// TODO 设计cookie
pub fn get_cookie(headers: &HeaderMap) -> Option<String> {
    let cookies = headers.typed_get::<Cookie>();
    if let Some(cookies) = cookies {
        return cookies.get(COOKIE_NAME).map(|value| value.to_string()); // TODO 注意and_then 和map的区别
    }

    None
}

pub fn set_cookie(value: &str) -> HeaderMap {
    let c = format!("{}={}", COOKIE_NAME, value);
    let mut hm = HeaderMap::new();
    hm.insert(axum::http::header::SET_COOKIE, (&c).parse().unwrap());
    hm
}
