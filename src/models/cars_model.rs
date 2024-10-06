use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Deserialize, Serialize)]
pub struct CarModelForCreate {
    pub brand: String,
    pub category_id: i64,
    pub color_i: i64,
    pub model: String,
    pub year: u32,
    pub fuel_type: String,
    pub engine_size: u32,
    pub transmission_type: String,
    pub vin: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize)]
pub struct CarModelForSelect {
    pub id: i64,
    pub brand: String,
    pub category_id: i64,
    pub color_i: i64,
    pub model: String,
    pub year: u32,
    pub fuel_type: String,
    pub engine_size: u32,
    pub transmission_type: String,
    pub vin: String,
    pub ctime: String,
    pub mtime: String,
}

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Deserialize, Serialize)]
pub struct CargModelForUpdate {
    pub id: i64,
    pub brand: String,
    pub category_id: i64,
    pub color_i: i64,
    pub model: String,
    pub year: u32,
    pub fuel_type: String,
    pub engine_size: u32,
    pub transmission_type: String,
    pub vin: String,
}
