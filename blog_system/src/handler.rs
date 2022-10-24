use axum::http::{header, HeaderMap, StatusCode};

pub(crate) type HtmlView = axum::response::Html<String>; // TODO 继承关系
pub type RedirectView = (StatusCode, HeaderMap, ());

pub fn redirect(url: &str) -> crate::Result<RedirectView> {
    // let mut header_map = HeaderMap::new();
    // header_map.append(header::LOCATION, url.parse().unwrap());
    // Ok((StatusCode::FOUND, header_map, ()))
    redirect_with_cookie(url, None)
}

pub fn redirect_with_cookie(url: &str, c: Option<&str>) -> crate::Result<RedirectView> {
    // TODO 搞懂重定向
    let mut hm = match c {
        Some(s) => crate::cookie::set_cookie(s),
        None => HeaderMap::new(),
    };
    hm.insert(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, hm, ()))
}
