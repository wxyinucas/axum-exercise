use axum::Json;

use crate::{structs::TodoResponse, Result};

pub async fn usage() -> Result<Json<TodoResponse<Vec<&'static str>>>> {
    let data = r#"
        ### USAGE ###
        - GET /todo -- get all todo list
        - POST /todo -- create a todo list
        - GET /todo/:list_id -- get detail for a todo list
        - DELETE /todo/:list_id -- delete a todo list, include it's items
        - PUT /todo/:list_id -- edit a todo list
        - GET /todo/:list_id/items -- get items from todo list
        - GET /todo/:list_id/items/:item_id -- get detail for a todo item
        - PUT /todo/:list_id/items/:item_id -- edit a todo item(set the item to checked)
        - DELETE /todo/:list_id/items/:item_id -- delete a todo item
    "#;

    let data = data
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let res = TodoResponse::ok(data);
    Ok(Json(res))
}

// =================================================================================================
//
// TodoList
//
// =================================================================================================
type TodoJsonResponse<T> = Result<Json<TodoResponse<T>>>;

pub mod todo_list {
    use axum::extract::Path;
    use axum::{Extension, Json};
    use sqlx::PgPool;
    use axum_macros::debug_handler;

    use crate::handlers::TodoJsonResponse;
    use crate::structs::{CreateTodoList, TodoList, TodoListID, TodoResponse, UpdateTodoList};
    use crate::{ListStore, Storage};

    #[debug_handler]
    pub async fn create(
        Extension(pool): Extension<PgPool>,
        Json(form): Json<CreateTodoList>,
    ) -> TodoJsonResponse<TodoListID> {
        let res = ListStore::new(pool).create(form).await?;
        Ok(Json(TodoResponse::ok(res)))
    }

    pub async fn get_all(Extension(pool): Extension<PgPool>) -> TodoJsonResponse<Vec<TodoList>> {
        let res = ListStore::new(pool).get_all().await?;
        Ok(Json(TodoResponse::ok(res)))
    }

    pub async fn find(
        Extension(pool): Extension<PgPool>,
        Path(id): Path<i32>,
    ) -> TodoJsonResponse<TodoList> {
        let res = ListStore::new(pool).find(id).await?;
        Ok(Json(TodoResponse::ok(res)))
    }
    
    pub async fn update(
        Extension(pool): Extension<PgPool>,
        Json(form): Json<UpdateTodoList>,
    ) -> TodoJsonResponse<bool> {
        let res = ListStore::new(pool).update(form).await?;
        Ok(Json(TodoResponse::ok(res)))
    }
    
    pub async fn delete(
        Extension(pool): Extension<PgPool>,
        Path(id): Path<i32>,
    ) -> TodoJsonResponse<bool> {
        let res = ListStore::new(pool).delete(id).await?;
        Ok(Json(TodoResponse::ok(res)))
    }
}
