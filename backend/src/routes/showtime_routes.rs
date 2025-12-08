use crate::handlers::showtime_handler;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn showtime_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route(
            "/api/showtimes",
            get(showtime_handler::get_all)
                .post(showtime_handler::create),
        )
        .route(
            "/api/showtimes/{id}",
            get(showtime_handler::get_by_id)
                .put(showtime_handler::update)
                .delete(showtime_handler::delete_showtime),
        )
        
        .route(
            "/api/showtimes/movie/{movie_id}",
            get(showtime_handler::get_by_movie),
        )
        .route(
            "/api/showtimes/studio/{studio_id}",
            get(showtime_handler::get_by_studio),
        )
        .route(
            "/api/showtimes/upcoming",
            get(showtime_handler::get_upcoming),
        )
        .route(
            "/api/showtimes/today",
            get(showtime_handler::get_today),
        )
        .route(
            "/api/showtimes/filter",
            get(showtime_handler::get_filtered),
        )
        .route(
            "/api/showtimes/movie/{movie_id}/date",
            get(showtime_handler::get_by_movie_and_date),
        )
}