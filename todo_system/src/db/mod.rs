use async_trait::async_trait;
use crate::{ Result};

mod todo_list;
pub use todo_list::ListStore;


#[async_trait]
pub trait Storage<CreateType, UpdateType>
// TODO: 注意抽象的等级，此处仅做对数据库操作的抽象，不做服务器封装！
{
    type OutputType;
    type IdType;

    async fn create(&self, form: CreateType) -> Result<Self::IdType>;

    async fn get_all(&self) -> Result<Vec<Self::OutputType>>; // todo 返回值是IntoResponse吗？

    async fn find(&self, id: i32) -> Result<Self::OutputType>;

    async fn update(&self, form: UpdateType) -> Result<bool>;

    async fn delete(&self, id: i32) -> Result<bool>;
}

#[cfg(test)]
mod tests {
    // todo 添加测试
    #[tokio::test]
    fn some_funcs() {
        todo!()
    }
}
