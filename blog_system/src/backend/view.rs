use askama::Template;

use crate::models::Category;

#[derive(Template)]
#[template(path = "backend/index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "backend/category/add.html")]
pub struct Add {}

#[derive(Template)]
#[template(path = "backend/category/index.html")]
pub struct CategoryIndex {
    pub list: Vec<Category>,
    pub msg: Option<String>,
}
#[derive(Template)]
#[template(path = "backend/category/edit.html")]
pub struct Edit {
    pub item: Category,
}
