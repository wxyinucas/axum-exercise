use async_trait::async_trait;
use axum::Extension;
use sqlx::PgPool;
use crate::{structs, Result};

mod todo_list;

#[async_trait]
pub trait Storage // TODO: 注意抽象的等级，此处仅做对数据库操作的抽象，不做服务器封装！
where
    Self: Sized, // todo notice here
{
    type IdType;
    type CreateType;
    type UpdateType;

    async fn create(
        Extension(pool): Extension<PgPool>,
        form: Self::CreateType,
    ) -> Result<Self::IdType>;

    async fn get_all(Extension(pool): Extension<PgPool>) -> Result<Vec<Self>>; // todo 返回值是IntoResponse吗？

    async fn find(Extension(pool): Extension<PgPool>, id: i32) -> Result<Self>;

    async fn update(Extension(pool): Extension<PgPool>, form: Self::UpdateType) -> Result<bool>;

    async fn delete(Extension(pool): Extension<PgPool>, id: i32) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    fn some_funcs() {
        todo!()
    }
}
