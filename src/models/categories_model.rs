use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Deserialize, Serialize)]
pub struct CategoryForCreate {
    pub cat_id: String,
    pub cat_name: String,
    pub drive_type: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Deserialize, Serialize)]
pub struct CategoryForSelect {
    pub id: i64,
    pub cat_id: String,
    pub cat_name: String,
    pub drive_type: String,
    pub ctime: String,
    pub mtime: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Deserialize, Serialize)]
pub struct CategoryForUpdate {
    pub id: i64,
    pub cat_id: String,
    pub cat_name: String,
    pub drive_type: String,
}
