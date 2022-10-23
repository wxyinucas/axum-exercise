use axum::http::{header, HeaderMap, StatusCode};

pub(crate) type HtmlView = axum::response::Html<String>; // todo 继承关系
pub type RedirectView = (StatusCode, HeaderMap, ());

pub fn redirect(url: &str) -> crate::Result<RedirectView> {
    let mut header_map = HeaderMap::new();
    header_map.append(header::LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, header_map, ()))
}
