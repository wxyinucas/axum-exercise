use askama::Template;
use axum::extract::{Path, Query};
use axum::response::Html;
use axum::{Extension, Form};
use sqlx::PgPool;

use crate::backend::view::category;
use crate::db::traits::{StorageCategory, StorageTopic};
use crate::form::{CreateTopic, EditTopic};
use crate::handler::HtmlView;
use crate::{redirect, BlogError, RedirectView, Result};

use super::super::handlers::Args;
use super::super::view::topic::{Add, Edit, Index};

pub async fn add_ui(Extension(pool): Extension<PgPool>) -> Result<HtmlView> {
    let cats = <PgPool as StorageCategory>::list(&pool)
        .await
        .map_err(BlogError::from)?;
    let tmpl = Add { cats };
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn edit_ui(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> Result<HtmlView> {
    let cats = <PgPool as StorageCategory>::list(&pool)
        .await
        .map_err(BlogError::from)?;
    let item = <PgPool as StorageTopic>::find2edit(&pool, id)
        .await
        .map_err(BlogError::from)?;
    let tmpl = Edit { cats, item };
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn add(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<CreateTopic>,
) -> Result<RedirectView> {
    <PgPool as StorageTopic>::create(&pool, &form)
        .await
        .map_err(BlogError::from)?;
    redirect("/admin/topic?msg=文章添加成功")
}

pub async fn index(
    Extension(pool): Extension<PgPool>,
    Query(args): Query<Args>,
) -> Result<HtmlView> {
    let page = args.page();
    let list = <PgPool as StorageTopic>::list(&pool, page)
        .await
        .map_err(BlogError::from)?;
    let tmpl = Index {
        msg: args.msg.clone(),
        page,
        list,
    };
    let res = tmpl.render().map_err(BlogError::from)?;
    Ok(Html(res))
}

pub async fn edit(
    Extension(pool): Extension<PgPool>,
    Form(form): Form<EditTopic>,
    Path(id): Path<i64>,
) -> Result<RedirectView> {
    <PgPool as StorageTopic>::update(&pool, &form, id).await?;
    redirect("/admin/topic?msg=文章修改成功")
}

pub async fn del(Extension(pool): Extension<PgPool>, Path(id): Path<i64>) -> Result<RedirectView> {
    <PgPool as StorageTopic>::del_or_restore(&pool, id, true).await?;
    redirect("/admin/topic?msg=文章删除成功") // todo 去哪了？为啥呀？
}
