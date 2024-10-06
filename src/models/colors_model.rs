use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ColorForCreate {
    pub color_id: String,
    pub color_name: String,
    pub color_type: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ColorForSelect {
    pub id: i64,
    pub color_id: String,
    pub color_name: String,
    pub color_type: String,
    pub ctime: String,
    pub mtime: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct ColorForUpdate {
    pub id: i64,
    pub color_id: String,
    pub color_name: String,
    pub color_type: String,
}
