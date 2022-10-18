
use askama::Template;

#[derive(Template)]
#[template(path = "frontend/base.html")]
pub struct Index;