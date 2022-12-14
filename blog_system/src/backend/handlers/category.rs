use askama::Template;
use axum::extract::{Path, Query};
use axum::response::Html;
use axum::{Extension, Form};
use sqlx::PgPool;

use crate::db::traits::StorageCategory;
use crate::form::EditCategory;
use crate::handler::HtmlView;
use crate::{form, redirect, BlogError, RedirectView, Result};

use super::super::handlers::Args;
use super::super::view::category::{Add, Index, Edit};

pub async fn add_ui() -> Result<HtmlView> {
    let tmpl = Add {};
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn edit_ui(Extension(pool): Extension<PgPool>, Path(id): Path<i32>) -> Result<HtmlView> {
    let item = <PgPool as StorageCategory>::find(&pool, id).await?;
    let tmpl = Edit { item };
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn add(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<form::CreateCategory>,
) -> Result<RedirectView> {
    <PgPool as StorageCategory>::create(&pool, &form).await?;
    redirect("/admin/category?msg=分类添加成功") // TODO 出现在哪里，怎么出现的？
}

pub async fn index(
    Extension(pool): Extension<PgPool>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let list = <PgPool as StorageCategory>::list(&pool).await?;
    let tmpl = Index {
        list,
        msg: args.msg,
    };
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn del(Extension(pool): Extension<PgPool>, Path(id): Path<i32>) -> Result<RedirectView> {
    <PgPool as StorageCategory>::del_or_restore(&pool, id, true).await?;
    redirect("/admin/category?msg=分类删除成功")
}

pub async fn edit(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<EditCategory>,
) -> Result<RedirectView> {
    <PgPool as StorageCategory>::edit(&pool, &form).await?;
    redirect("/admin/category?msg=分类修改成功")
}
