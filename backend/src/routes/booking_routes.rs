use axum::{routing::{get, post, put}, Router};
use std::sync::Arc;
use crate::config::DatabasePools;
use crate::handlers::booking_handler::*;

pub fn booking_routes() -> Router<Arc<DatabasePools>> {
    Router::new()
        .route("/api/bookings", get(get_all_bookings).post(create_booking))
        .route("/api/bookings/{id}", get(get_booking_by_id))
        .route("/api/bookings/user/{user_id}", get(get_bookings_by_user))
        .route("/api/bookings/{id}/payment", put(update_payment_status))
        .route("/api/bookings/{id}/cancel", put(cancel_booking))
        .route("/api/bookings/showtime/{showtime_id}/seats", get(get_booked_seats_by_showtime))
}
