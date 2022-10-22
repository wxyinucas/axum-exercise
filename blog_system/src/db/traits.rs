use axum::async_trait;

use crate::Result;
use crate::{form, models};

#[async_trait]
pub trait StorageCategory {
    async fn create(&self, form: &form::CreateCategory) -> Result<models::CategoryID>;
    async fn list(&self) -> Result<Vec<models::Category>>;
    async fn del_or_restore(&self, id: i32, is_del: bool) -> Result<bool>;
    async fn edit(&self, form: &form::EditCategory) -> Result<bool>;
    async fn find(&self, id: i32) -> Result<models::Category>;
}
