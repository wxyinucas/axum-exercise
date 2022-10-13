pub use errors::TodoError;
pub use handlers::*;
pub use structs::{Config, TodoResponse};

mod errors;
mod handlers;
mod structs;

pub type Result<T> = std::result::Result<T, TodoError>;
