use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Seat {
    pub id: i64,
    pub studio_id: i64,
    pub seat_code: String,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub seat_status: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct SeatWithBookingStatus {
    pub id: i64,
    pub studio_id: i64,
    pub seat_code: String,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub seat_status: Option<String>,
    pub is_booked: bool,
    pub booking_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct GenerateSeatsRequest {
    pub studio_id: i64,
    pub rows: i32,          
    pub seats_per_row: i32, 
}

#[derive(Serialize)]
pub struct GenerateSeatsResponse {
    pub studio_id: i64,
    pub total_seats_created: i32,
}
