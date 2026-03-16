use serde::{Deserialize, Serialize};

pub type BlockApp = BlockedApp;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct BlockedApp {
    pub(crate) id: String,
    pub(crate) package_name: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Dictionary {
    pub(crate) string: String,
    pub(crate) id: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Note {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) last_modified: i64,
}

#[derive(Deserialize)]
pub struct Pagination {
    pub(crate) page_id: String,
    pub(crate) page_size: i32,
}

#[derive(Deserialize, Serialize)]
pub struct IdDTO {
    pub(crate) id: String,
}
