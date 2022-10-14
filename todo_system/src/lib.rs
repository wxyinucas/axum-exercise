pub use errors::TodoError;
pub use handlers::*;

mod db;
mod errors;
mod handlers;
pub mod structs;

pub type Result<T> = std::result::Result<T, TodoError>;
pub use db::Storage;
