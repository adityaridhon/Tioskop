use crate::handlers::booking_handler::*;
use axum::{
    Router,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn booking_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/bookings", get(get_all).post(create))
        .route("/api/bookings/{id}", get(get_by_id))
        .route("/api/bookings/user/{user_id}", get(get_by_user))
        .route("/api/bookings/{id}/payment", put(update_payment))
        .route("/api/bookings/{id}/cancel", put(cancel))
        .route(
            "/api/bookings/showtime/{showtime_id}/seats",
            get(get_booked_seats),
        )
}
