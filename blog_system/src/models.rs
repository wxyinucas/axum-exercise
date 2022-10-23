use std::time;

use chrono::{Local, TimeZone};
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub is_del: bool,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct CategoryID {
    pub id: i32,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TopicList {
    pub id: i64,
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub hit: i32,
    // pub dateline: time::SystemTime, // todo  新处理方式
    pub dateline: String,
    pub is_del: bool,
    pub category_name: String,
}

impl TopicList {
    pub fn dateline(&self) -> String {
        // TODO NOTICE
        // let ts = self
        //     .dateline
        //     .clone()
        //     .duration_since(time::UNIX_EPOCH)
        //     .unwrap_or(time::Duration::from_secs(0))
        //     .as_secs() as i64;
        // Local
        //     .timestamp(ts, 0)
        //     .format("%Y/%m/%d %H:%M:%S")
        //     .to_string()
        self.dateline.clone()
    }
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TopicID {
    pub id: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TopicEditData {
    pub id: i64,
    pub title: String,
    pub category_id: i32,
    pub summary: String,
    pub markdown: String,
}
