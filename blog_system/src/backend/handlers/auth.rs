use askama::Template;
use axum::response::Html;
use axum::{Extension, Form};
use sqlx::PgPool;

use crate::db::traits::StorageAdmin;
use crate::form::AdminLogin;
use crate::handler::{HtmlView, redirect_with_cookie};
use crate::{password, redirect, BlogError, RedirectView, Result};

use super::super::view::auth::Login;

pub async fn login_ui() -> Result<HtmlView> {
    let tmpl = Login {};
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn login(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<AdminLogin>,
) -> Result<RedirectView> {
    tracing::info!("{}", crate::password::hash("axum.rs")?);
    let admin_info = <PgPool as StorageAdmin>::find(&pool, &form.email)
        .await
        .map_err(|err| match err {
            BlogError::NotFoundError(_) => BlogError::IncorrectLoginError,
            _ => err,
        })?; //TODO: 用span记录信息，event只是触发

    let hashed_pwd = crate::password::hash(&admin_info.password)?;
    // let hashed_pwd = "select";
    let verify = password::verify(&form.password, &hashed_pwd)?;
    if !verify {
        return Err(BlogError::IncorrectLoginError);
    }

    redirect_with_cookie("/admin", Some(&admin_info.email))
}

pub async fn logout() -> Result<RedirectView> {
    redirect_with_cookie("/auth", Some(""))
}
