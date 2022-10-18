use askama::Template;
use axum::response::Html;
use axum_macros::debug_handler;

use crate::handler::HtmlView;
use crate::BlogError;

#[debug_handler]
pub async fn index() -> crate::Result<HtmlView> {
    let tmpl = crate::view::backend::index::Index;
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}
