use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub web_config: WebConfig,
    pub postgres_config: PostgresConfig,
}

#[derive(Debug, Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Debug, Deserialize)]
pub struct PostgresConfig {
    domain: String,
    db_name: String,
}

impl PostgresConfig {
    pub fn make_address(&self) -> String {
        format!("{}/{}", self.domain, self.db_name)
    }
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let config: Config =
            toml::from_str(include_str!("configs.toml")).map_err(|err| err.to_string())?;
        Ok(config)
    }
}

// =================================================================================================
//
// Response
//
// =================================================================================================
#[derive(Debug, Deserialize, Serialize)]
pub struct TodoResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

impl<T> TodoResponse<T>
where
    T: Serialize,
{
    pub fn new(code: i32, msg: String, data: Option<T>) -> Self {
        Self { code, msg, data }
    }

    pub fn ok(data: T) -> Self {
        Self::new(StatusCode::OK.as_u16() as i32, "OK".to_string(), Some(data))
    }

    pub fn err(code: i32, msg: String) -> Self {
        Self::new(code, msg, None)
    }
}

// =================================================================================================
//
// Todo System
//
// =================================================================================================
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TodoListID {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoList {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoList {
    pub id: i32,
    pub title: String,
}


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TodoItem{
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct TodoItemID{
    pub id: i32,
    pub list_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoItem {
    pub title: String,
    pub list_id: i32,
}


#[derive(Debug, Deserialize)]
pub struct UpdateTodoItem{
    pub id: i32,
    pub list_id: i32,
}
