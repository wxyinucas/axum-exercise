// TODO 为了什么，做了什么，怎么做的？
use bcrypt::DEFAULT_COST;

use crate::{BlogError, Result};

pub fn hash(pwd: &str) -> Result<String> {
    bcrypt::hash(pwd, DEFAULT_COST).map_err(BlogError::from)
}

pub fn verify(pwd: &str, hashed_pwd: &str) -> Result<bool> {
    bcrypt::verify(pwd, hashed_pwd).map_err(BlogError::from)
}
