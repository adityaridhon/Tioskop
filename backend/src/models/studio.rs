use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Studio {
    pub id: i64,
    pub cinema_id: Option<i64>,
    pub name: String,
    pub capacity: i32,
    pub r#type: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Cinema {
    pub id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub user_id: Option<i64>,
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
