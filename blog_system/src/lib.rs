pub use error::BlogError;

mod error;
mod db;

pub type Result<T> = std::result::Result<T, BlogError>;
