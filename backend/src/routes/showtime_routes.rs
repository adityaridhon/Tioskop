use axum::{routing::{get, post, put, delete}, Router};
use sqlx::MySqlPool;
use crate::handlers::showtime_handler::*;

pub fn showtime_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/showtimes", get(get_all_showtimes).post(create_showtime))
        .route("/api/showtimes/movie/{movie_id}", get(get_showtimes_by_movie))
        .route("/api/showtimes/{id}", put(update_showtime).delete(delete_showtime))
}