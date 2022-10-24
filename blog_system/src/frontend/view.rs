use askama::Template;
use crate::db::Paginate;
use crate::models::{Category, TopicList};

#[derive(Template)]
#[template(path="frontend/index.html")]
pub struct Index {}