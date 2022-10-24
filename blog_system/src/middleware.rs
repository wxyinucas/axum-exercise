use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};

use crate::BlogError;

pub struct Auth(pub String); // TODO 这个中间件又做了什么呢？

#[async_trait]
impl<B> FromRequest<B> for Auth
where
    B: Send,
{
    type Rejection = BlogError;
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let headers = req.headers();
        let cookie = crate::cookie::get_cookie(headers);
        let auth = cookie.unwrap_or("".to_string());
        if auth.is_empty() {
            return Err(BlogError::ForbiddenError);
        }
        Ok(Auth(auth))
    }
}
