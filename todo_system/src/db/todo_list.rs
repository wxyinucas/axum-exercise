use async_trait::async_trait;
use sqlx::{Executor, PgPool, Postgres};
use tracing::debug;

use crate::structs::{CreateTodoList, TodoList, TodoListID, UpdateTodoList};
use crate::TodoError;

use super::Storage;

pub struct ListStore {
    pool: PgPool,
}

impl ListStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AsRef<PgPool> for ListStore {
    // TODO 注意此处的实现！！如何理解，与deref有什么区别？？
    fn as_ref(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
// impl Storage<CreateTodoList, UpdateTodoList> for PgPool {
impl Storage for ListStore {
    type CreateType = CreateTodoList;
    type UpdateType = UpdateTodoList;
    type OutputType = TodoList;
    type IdType = TodoListID;

    async fn create(&self, form: Self::CreateType) -> crate::Result<Self::IdType> {
        let sql = "INSERT INTO todo_list (title) VALUES ($1) RETURNING id";
        let res = sqlx::query_as::<Postgres, Self::IdType>(sql)
            .bind(form.title)
            .fetch_one(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn get_all(&self) -> crate::Result<Vec<Self::OutputType>> {
        let sql = "SELECT id, title FROM todo_list ORDER BY id DESC";
        let res = sqlx::query_as::<Postgres, Self::OutputType>(sql)
            .fetch_all(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn find(&self, id:Self::IdType) -> crate::Result<Self::OutputType> {
        let sql = "SELECT id,title FROM todo_list WHERE id=$1 LIMIT 1";
        let res = sqlx::query_as::<Postgres, Self::OutputType>(sql)
            .bind(id.id)
            .fetch_one(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn update(&self, form: Self::UpdateType) -> crate::Result<bool> {
        debug!("{:?}", form);
        let sql = "UPDATE todo_list SET title=$1 WHERE id=$2";
        let res = sqlx::query(sql)
            .bind(form.title)
            .bind(form.id)
            .execute(self.as_ref())
            .await
            .map_err(TodoError::from)?;

        Ok(res.rows_affected() > 0)
    }

    async fn delete(&self, form: Self::UpdateType) -> crate::Result<bool> {
        let mut tx = self.as_ref().begin().await.map_err(TodoError::from)?;

        let sql = "DELETE FROM todo_list  WHERE id=$1";
        let query = sqlx::query(sql).bind(form.id);
        let res = tx.execute(query).await.map_err(TodoError::from);
        if let Err(err) = res {
            tx.rollback().await.map_err(TodoError::from)?;
            return Err(TodoError::from(err));
        };

        let sql = "DELETE FROM todo_item WHERE list_id=$1";
        let query = sqlx::query(sql).bind(form.id);
        let res = tx.execute(query).await.map_err(TodoError::from);
        if let Err(err) = res {
            tx.rollback().await.map_err(TodoError::from)?;
            return Err(TodoError::from(err));
        }
        tx.commit().await.map_err(TodoError::from)?;
        Ok(true)
    }
}
