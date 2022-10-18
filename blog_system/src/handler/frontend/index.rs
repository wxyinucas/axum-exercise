use axum::response::Html;
use askama::Template;
use crate::handler::HtmlView;
use crate::BlogError;

pub async fn index() -> crate::Result<HtmlView> {
    let tmpl = crate::view::frontend::index::Index;
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}
