use axum::async_trait;

use crate::models::{TopicEditData, TopicList};
use crate::Result;
use crate::{form, models};

use super::structs::Paginate;

#[async_trait]
pub trait StorageCategory {
    async fn create(&self, form: &form::CreateCategory) -> Result<models::CategoryID>;
    async fn list(&self) -> Result<Vec<models::Category>>;
    async fn del_or_restore(&self, id: i32, is_del: bool) -> Result<bool>;
    async fn edit(&self, form: &form::EditCategory) -> Result<bool>;
    async fn find(&self, id: i32) -> Result<models::Category>;
}

#[async_trait]
pub trait StorageTopic {
    // todo: 先model，再form再trait。
    async fn create(&self, form: &form::CreateTopic) -> Result<models::TopicID>;
    async fn list(&self, page: u32) -> Result<Paginate<Vec<TopicList>>>;
    async fn update(&self, form: &form::EditTopic, id: i64) -> Result<bool>;
    async fn find2edit(&self, id: i64) -> Result<TopicEditData>;
    async fn del_or_restore(&self, id: i64, is_del: bool) -> Result<bool>;
}
