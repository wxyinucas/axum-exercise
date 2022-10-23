use axum::async_trait;
use sqlx::{PgPool, Postgres, Row};

use crate::db::traits::StorageCategory;
use crate::form::{CreateCategory, EditCategory};
use crate::models::{Category, CategoryID};
use crate::BlogError;

// pub struct PGStore {
//     pool: PgPool,
// }
// 
// impl PGStore {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }
// }
// 
// impl AsRef<PgPool> for PGStore {
//     // TODO 注意此处的实现！！如何理解，与deref有什么区别？？
//     fn as_ref(&self) -> &PgPool {
//         &self.pool
//     }
// }

#[async_trait]
impl StorageCategory for PgPool {
    async fn create(&self, form: &CreateCategory) -> crate::Result<CategoryID> {
        let sql = "SELECT * FROM categories WHERE name=$1"; // todo count(*) https://github.com/launchbadge/sqlx
        let n = sqlx::query(sql)
            .bind(&form.name)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        if n.rows_affected() > 0 {
            return Err(BlogError::DuplicateError(
                "Same Category exists".to_string(),
            ));
        }

        let sql = "INSERT INTO categories (name, is_del) VALUES ($1, false) RETURNING id";
        let res = sqlx::query_as::<Postgres, CategoryID>(sql)
            .bind(&form.name)
            .fetch_one(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res)
    }

    async fn list(&self) -> crate::Result<Vec<Category>> {
        let sql = "SELECT id, name, is_del FROM categories WHERE is_del = false ORDER BY id ASC LIMIT 1000";
        let res = sqlx::query_as::<Postgres, Category>(sql)
            .fetch_all(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res)
    }

    async fn del_or_restore(&self, id: i32, is_del: bool) -> crate::Result<bool> {
        let sql = "UPDATE categories SET is_del=$1 WHERE id=$2";
        let n = sqlx::query(sql)
            .bind(is_del)
            .bind(id)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        Ok(n.rows_affected() > 0)
    }

    async fn edit(&self, form: &EditCategory) -> crate::Result<bool> {
        let sql = "SELECT * FROM categories WHERE name=$1"; // todo count(*) https://github.com/launchbadge/sqlx
        let n = sqlx::query(sql)
            .bind(&form.name)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        if n.rows_affected() > 0 {
            return Err(BlogError::DuplicateError(
                "Same Category exists".to_string(),
            ));
        }

        let sql = "UPDATE categories SET name=$1 WHERE id=$2";
        let res = sqlx::query(sql)
            .bind(&form.name)
            .bind(&form.id)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res.rows_affected() > 0)
    }

    async fn find(&self, id: i32) -> crate::Result<Category> {
        let sql = "SELECT id, name, is_del FROM categories WHERE id=$1";
        let res = sqlx::query_as::<Postgres, Category>(sql)
            .bind(id)
            .fetch_one(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res)
    }
}

// todo 随时test，并能看到打印出的结果