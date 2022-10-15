use async_trait::async_trait;

pub use todo_item::ItemStore;
pub use todo_list::ListStore;

use crate::Result;

mod todo_item;
mod todo_list;

#[async_trait]
pub trait Storage
// TODO: 注意抽象的等级，此处仅做对数据库操作的抽象，不做服务器封装！
// todo: 考虑泛型和关联类型的设计区别
{
    type CreateType;
    type UpdateType;
    type OutputType;
    type IdType;

    async fn create(&self, form: Self::CreateType) -> Result<Self::IdType>;

    async fn get_all(&self) -> Result<Vec<Self::OutputType>>; // todo 返回值是IntoResponse吗？

    async fn find(&self, id: Self::IdType) -> Result<Self::OutputType>;

    async fn update(&self, form: Self::UpdateType) -> Result<bool>;

    async fn delete(&self, form: Self::UpdateType) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    // todo 添加测试
    #[tokio::test]
    fn some_funcs() {
        todo!()
    }
}
