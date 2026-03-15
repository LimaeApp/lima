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
