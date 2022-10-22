use askama::Template;
use axum::response::Html;

use crate::backend::view::Index;
use crate::handler::HtmlView;
use crate::{BlogError, Result};

pub async fn index() -> Result<HtmlView> {
    let tmpl = Index {};
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}
