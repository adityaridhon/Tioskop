use crate::handlers::seat_handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn seat_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/seats", get(get_all_seats))
        .route("/api/seats/generate", post(generate_seats_for_studio))
        .route("/api/seats/studio/{studio_id}", get(get_seats_by_studio))
        .route(
            "/api/seats/showtime/{showtime_id}",
            get(get_seats_by_showtime),
        )
        .route(
            "/api/seats/showtime/{showtime_id}/available",
            get(get_available_seats_by_showtime),
        )
}
