use serde::Deserialize;

pub(crate) mod auth;
pub(crate) mod category;
pub(crate) mod index;
pub(crate) mod topic;
// mod article;

#[derive(Deserialize)]
pub struct Args {
    pub msg: Option<String>,
    pub page: Option<u32>,
}
impl Args {
    pub fn msg(&self) -> String {
        self.msg.clone().unwrap_or("".to_string())
    }
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(0)
    }
}
