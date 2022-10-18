pub use error::BlogError;

pub(crate) mod db;
mod error;
pub mod handler;
pub(crate) mod view;

pub type Result<T> = std::result::Result<T, BlogError>;
