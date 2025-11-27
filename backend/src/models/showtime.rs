use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Showtime {
    pub id: i64,
    pub movie_id: Option<i64>,
    pub studio_id: Option<i64>,
    pub start_time: Option<chrono::NaiveDateTime>,
    pub price: Option<rust_decimal::Decimal>,
}

#[derive(Deserialize)]
pub struct CreateShowtimeRequest {
    pub movie_id: i64,
    pub studio_id: i64,
    pub start_time: String,  // Accept string instead of NaiveDateTime
    pub price: String,       // Accept string for Decimal
}

#[derive(Deserialize)]
pub struct UpdateShowtimeRequest {
    pub movie_id: Option<i64>,
    pub studio_id: Option<i64>,
    pub start_time: Option<String>,  // Accept string instead of NaiveDateTime
    pub price: Option<String>,       // Accept string for Decimal
}