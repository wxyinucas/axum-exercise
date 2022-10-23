use std::time;

use axum::async_trait;
use sqlx::{PgPool, Postgres};

use crate::form::{CreateCategory, CreateTopic, EditCategory, EditTopic};
use crate::md::to_html;
use crate::models::{Category, CategoryID, TopicEditData, TopicID, TopicList};
use crate::BlogError;

use super::structs::Paginate;
use super::traits::{StorageCategory, StorageTopic};
use super::{structs::pagination, DEFAULT_PAGE_SIZE};

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

#[async_trait]
impl StorageTopic for PgPool {
    async fn create(&self, form: &CreateTopic) -> crate::Result<TopicID> {
        let html = to_html(&form.markdown);
        let dateline = format!("{:?}", time::SystemTime::now());
        // let dateline = time::SystemTime::now();

        let sql = "INSERT INTO topics (title, category_id, summary, markdown, html, hit, dateline, is_del) VALUES ($1, $2, $3, $4, $5, 0,$6, false) RETURNING id";
        let res = sqlx::query_as::<_, TopicID>(sql)
            .bind(&form.title)
            .bind(&form.category_id)
            .bind(&form.summary)
            .bind(&form.markdown)
            .bind(&html)
            .bind(&dateline)
            .fetch_one(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res)
    }

    async fn list(&self, page: u32) -> crate::Result<Paginate<Vec<TopicList>>> {
        let sql=format!(
            "SELECT id,title,category_id,summary,hit,dateline,is_del,category_name FROM v_topic_cat_list WHERE is_del=false ORDER BY id DESC LIMIT {} OFFSET {}",
            DEFAULT_PAGE_SIZE, DEFAULT_PAGE_SIZE as u32 * page);
        let data = sqlx::query_as::<_, TopicList>(&sql)
            .fetch_all(self)
            .await
            .map_err(BlogError::from)?;

        // todo count(*)
        let sql = "SELECT * FROM v_topic_cat_list WHERE is_del=false";
        let total_records = sqlx::query(sql)
            .execute(self)
            .await
            .map_err(BlogError::from)?
            .rows_affected() as i64;
        pagination(data, total_records, page).await
    }

    async fn update(&self, form: &EditTopic, id: i64) -> crate::Result<bool> {
        let html = to_html(&form.markdown);
        let sql = "UPDATE topics SET title=$1, category_id=$2, summary=$3, markdown=$4, html=$5 WHERE id=$6";
        let n = sqlx::query(sql)
            .bind(&form.title)
            .bind(&form.category_id)
            .bind(&form.summary)
            .bind(&form.markdown)
            .bind(&html)
            .bind(id)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        Ok(n.rows_affected() > 0)
    }

    async fn find2edit(&self, id: i64) -> crate::Result<TopicEditData> {
        let sql =
            "SELECT id, title, category_id, summary, markdown FROM topics WHERE id=$1 LIMIT 1";
        let res = sqlx::query_as::<_, TopicEditData>(sql)
            .bind(id)
            .fetch_one(self)
            .await
            .map_err(BlogError::from)?;
        Ok(res)
    }

    async fn del_or_restore(&self, id: i64, is_del: bool) -> crate::Result<bool> {
        let sql = "UPDATE topics SET is_del=$1 WHERE id=$2";
        let n = sqlx::query(sql)
            .bind(is_del)
            .bind(id)
            .execute(self)
            .await
            .map_err(BlogError::from)?;
        Ok(n.rows_affected() > 0)
    }
}
