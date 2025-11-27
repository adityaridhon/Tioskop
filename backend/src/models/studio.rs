use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Studio {
    pub id: i64,
    pub cinema_id: Option<i64>,
    pub name: String,
    pub capacity: i32,
    pub r#type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateStudioRequest {
    pub cinema_id: i64,
    pub name: String,
    pub capacity: i32,
    pub r#type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateStudioRequest {
    pub cinema_id: Option<i64>,
    pub name: Option<String>,
    pub capacity: Option<i32>,
    pub r#type: Option<String>,
}
