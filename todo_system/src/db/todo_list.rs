use async_trait::async_trait;
use axum::Extension;
use sqlx::{Executor, PgPool, Postgres};

use crate::structs::{CreateTodoList, TodoList, TodoListID, UpdateTodoList};
use crate::TodoError;

use super::Storage;

#[async_trait]
impl Storage for TodoList {
    type IdType = TodoListID;
    type CreateType = CreateTodoList;
    type UpdateType = UpdateTodoList;

    async fn create(
        Extension(pool): Extension<PgPool>,
        form: Self::CreateType,
    ) -> crate::Result<Self::IdType> {
        let sql = "INSERT INTO todo_list (title) VALUES ($1) RETURNING id";
        let res = sqlx::query_as::<Postgres, Self::IdType>(sql)
            .bind(form.title)
            .fetch_one(&pool)
            .await
            .map_err(TodoError::from);
        res
    }

    async fn get_all(Extension(pool): Extension<PgPool>) -> crate::Result<Vec<Self>> {
        let sql = "SELECT id, title FROM todo_list ORDER BY id DESC";
        let res = sqlx::query_as::<Postgres, Self>(sql)
            .fetch_all(&pool)
            .await
            .map_err(TodoError::from);
        res
    }

    async fn find(Extension(pool): Extension<PgPool>, id: i32) -> crate::Result<Self> {
        let sql = "SELECT id,title FROM todo_list WHERE id=$1 LIMIT 1";
        let res = sqlx::query_as::<Postgres, Self>(sql)
            .bind(id)
            .fetch_one(&pool)
            .await
            .map_err(TodoError::from);
        res
    }

    async fn update(
        Extension(pool): Extension<PgPool>,
        form: Self::UpdateType,
    ) -> crate::Result<bool> {
        let sql = "UPDATE todo_list SET title=$1 WHERE id=$2";
        let res = sqlx::query(sql)
            .bind(form.title)
            .bind(form.id)
            .execute(&pool)
            .await
            .map_err(TodoError::from)?;

        Ok(res.rows_affected() > 0)
    }

    async fn delete(Extension(pool): Extension<PgPool>, id: i32) -> crate::Result<bool> {
        let mut tx = pool.begin().await.map_err(TodoError::from)?;

        let sql = "DELETE FROM todo_list  WHERE id=$1";
        let query = sqlx::query(sql).bind(id);
        let res = tx.execute(query).await.map_err(TodoError::from);
        if let Err(err) = res {
            tx.rollback().await.map_err(TodoError::from)?;
            return Err(TodoError::from(err));
        };

        let sql = "DELETE FROM todo_item WHERE list_id=$1";
        let query = sqlx::query(sql).bind(id);
        let res = tx.execute(query).await.map_err(TodoError::from);
        if let Err(err) = res {
            tx.rollback().await.map_err(TodoError::from)?;
            return Err(TodoError::from(err));
        }
        tx.commit().await.map_err(TodoError::from)?;
        Ok(true)
    }
}
