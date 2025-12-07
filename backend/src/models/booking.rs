use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
pub struct Booking {
    pub id: i64,
    pub user_id: Option<i64>,
    pub showtime_id: Option<i64>,
    pub booking_code: Option<String>,
    pub total_price: Option<rust_decimal::Decimal>,
    pub payment_status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Clone)]
pub struct BookingSeat {
    pub id: i64,
    pub booking_id: Option<i64>,
    pub seat_id: Option<i64>,
    pub price: Option<rust_decimal::Decimal>,
}

#[derive(Deserialize)]
pub struct CreateBookingRequest {
    pub showtime_id: i64,
    pub seat_ids: Vec<i64>,
}

#[derive(Deserialize)]
pub struct UpdatePaymentStatusRequest {
    pub payment_status: String,
}

#[derive(Serialize)]
pub struct BookingDetail {
    pub id: i64,
    pub user_id: Option<i64>,
    pub showtime_id: Option<i64>,
    pub booking_code: Option<String>,
    pub total_price: Option<rust_decimal::Decimal>,
    pub payment_status: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub seats: Vec<BookingSeatDetail>,
}

#[derive(Serialize, Clone)]
pub struct BookingSeatDetail {
    pub seat_id: i64,
    pub seat_code: String,
    pub price: Option<rust_decimal::Decimal>,
}
