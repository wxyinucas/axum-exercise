pub use config::Config;
pub use error::BlogError;
pub use handler::{redirect, RedirectView};

pub mod backend;
mod config;
mod cookie;
pub(crate) mod db;
mod error;
pub(crate) mod form;
pub mod frontend;
pub(crate) mod handler;
pub(crate) mod md;
pub mod middleware;
pub(crate) mod models;
pub(crate) mod password;

pub type Result<T> = std::result::Result<T, BlogError>;
