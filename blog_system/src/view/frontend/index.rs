
use askama::Template;

#[derive(Template)]
#[template(path = "frontend/base.html")]
pub struct Index;
// todo 网页结构的具体功用