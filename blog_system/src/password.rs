// TODO 为了什么，做了什么，怎么做的？
use bcrypt::DEFAULT_COST;
use tracing::instrument;

use crate::{BlogError, Result};

pub fn hash(pwd: &str) -> Result<String> {
    bcrypt::hash(pwd, DEFAULT_COST).map_err(BlogError::make_into)
}

#[instrument]
pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    tracing::info!(path = module_path!(), line = line!()); // TODO new 可以根据这个制作新error tracing subscriber。
    bcrypt::verify(pwd, hashed_pwd).map_err(BlogError::make_into)
}
