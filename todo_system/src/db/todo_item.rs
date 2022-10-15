use async_trait::async_trait;
use sqlx::{PgPool, Postgres};

use crate::structs::{CreateTodoItem, TodoItem, TodoItemID, UpdateTodoItem};
use crate::{Storage, TodoError};

pub struct ItemStore {
    pool: PgPool,
}

impl ItemStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl AsRef<PgPool> for ItemStore {
    fn as_ref(&self) -> &PgPool {
        &self.pool
    }
}

#[async_trait]
impl Storage<CreateTodoItem, UpdateTodoItem> for ItemStore {
    type OutputType = TodoItem;
    type IdType = TodoItemID;

    async fn create(&self, form: CreateTodoItem) -> crate::Result<Self::IdType> {
        let sql = "INSERT INTO todo_item (title, list_id) VALUES ($1, $2) RETURNING id";
        let res = sqlx::query_as::<Postgres, Self::IdType>(sql)
            .bind(form.title)
            .bind(form.list_id)
            .fetch_one(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn get_all(&self) -> crate::Result<Vec<Self::OutputType>> {
        let sql = "SELECT * FROM todo_item ORDER BY id ASC";
        let res = sqlx::query_as::<Postgres, Self::OutputType>(sql)
            .fetch_all(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn find(&self, id: i32) -> crate::Result<Self::OutputType> {
        let sql = "SELECT * FROM todo_item WHERE id = $1";
        let res = sqlx::query_as::<Postgres, Self::OutputType>(sql)
            .bind(id)
            .fetch_one(self.as_ref())
            .await
            .map_err(TodoError::from);
        res
    }

    async fn update(&self, form: UpdateTodoItem) -> crate::Result<bool> {
        let sql =
            "UPDATE todo_item SET checked=true WHERE id=$1 AND list_id = $2 AND checked=false";
        let res = sqlx::query(sql)
            .bind(form.id)
            .bind(form.list_id)
            .execute(self.as_ref())
            .await
            .map_err(TodoError::from)?;
        Ok(res.rows_affected() > 0)
    }

    async fn delete(&self, form: UpdateTodoItem) -> crate::Result<bool> {
        let sql = "DELETE FROM todo_item WHERE id=$1 AND list_id = $2";
        let res = sqlx::query(sql)
            .bind(form.id)
            .bind(form.list_id)
            .execute(self.as_ref())
            .await
            .map_err(TodoError::from)?;

        Ok(res.rows_affected() > 0)
    }
}
