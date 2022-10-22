use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CategoryID {
    pub id: i32,
}
