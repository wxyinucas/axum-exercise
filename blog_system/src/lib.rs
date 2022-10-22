pub use config::Config;
pub use error::BlogError;

mod config;
pub(crate) mod db;
mod error;
pub(crate) mod form;
pub(crate) mod models;
pub mod backend;
pub mod frontend;
pub(crate) mod handler;

pub type Result<T> = std::result::Result<T, BlogError>;
