
use askama::Template;

#[derive(Template)]
#[template(path = "fronted/base.html")]
pub struct Index;