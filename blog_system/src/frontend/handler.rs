use crate::frontend::view::Index;
use crate::{BlogError, Result};
use askama::Template;
use axum::response::Html;

pub async fn index() -> Result<crate::handler::HtmlView> {
    let tmpl = Index {};
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}
