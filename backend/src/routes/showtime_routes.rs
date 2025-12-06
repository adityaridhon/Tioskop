use crate::handlers::showtime_handler::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::MySqlPool;

pub fn showtime_routes() -> Router<MySqlPool> {
    Router::new()
        .route(
            "/api/showtimes",
            get(get_all_showtimes).post(create_showtime),
        )
        .route(
            "/api/showtimes/movie/{movie_id}",
            get(get_showtimes_by_movie),
        )
        .route(
            "/api/showtimes/{id}",
            put(update_showtime).delete(delete_showtime),
        )
}
