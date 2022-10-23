use askama::Template;

#[derive(Template)]
#[template(path = "backend/index.html")]
pub struct Index {}

pub mod category {
    use askama::Template;

    use crate::models::Category;

    #[derive(Template)]
    #[template(path = "backend/category/add.html")]
    pub struct Add {}

    #[derive(Template)]
    #[template(path = "backend/category/index.html")]
    pub struct Index {
        pub list: Vec<Category>,
        pub msg: Option<String>,
    }

    #[derive(Template)]
    #[template(path = "backend/category/edit.html")]
    pub struct Edit {
        pub item: Category,
    }
}

pub mod topic {
    use askama::Template;

    use crate::db::Paginate;
    use crate::models::{Category, TopicEditData, TopicList};

    #[derive(Template)]
    #[template(path = "backend/topic/add.html")]
    pub struct Add {
        pub cats: Vec<Category>,
    }
    #[derive(Template)]
    #[template(path = "backend/topic/index.html")]
    pub struct Index {
        pub msg: Option<String>,
        pub page: u32,
        pub list: Paginate<Vec<TopicList>>,
    }

    #[derive(Template)]
    #[template(path = "backend/topic/edit.html")]
    pub struct Edit {
        pub cats: Vec<Category>,
        pub item: TopicEditData,
    }
}
