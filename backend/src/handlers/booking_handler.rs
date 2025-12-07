use crate::middleware::auth::AuthUser;
use crate::models::{booking::*, response::ApiResponse};
use crate::services::booking;
use axum::{extract::{Path, State}, Json};
use sea_orm::DatabaseConnection;

fn to_response<T>(result: Result<T, booking::BookingError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Booking>>> {
    to_response(booking::get_all(&db).await)
}

pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<BookingDetail>> {
    to_response(booking::get_detail(&db, id).await)
}

pub async fn get_by_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
) -> Json<ApiResponse<Vec<crate::entities::Booking>>> {
    to_response(booking::get_by_user(&db, user_id).await)
}

pub async fn create(
    AuthUser(user_id): AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateBookingRequest>,
) -> Json<ApiResponse<BookingDetail>> {
    to_response(booking::create(&db, user_id, payload).await)
}

pub async fn update_payment(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePaymentStatusRequest>,
) -> Json<ApiResponse<crate::entities::Booking>> {
    to_response(booking::update_payment(&db, id, payload.payment_status).await)
}

pub async fn cancel(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<crate::entities::Booking>> {
    to_response(booking::cancel(&db, id).await)
}

pub async fn get_booked_seats(
    State(db): State<DatabaseConnection>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<String>>> {
    to_response(booking::get_booked_seats(&db, showtime_id).await)
}