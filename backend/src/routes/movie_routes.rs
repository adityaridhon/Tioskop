use crate::handlers::movie_handler::*;
use crate::handlers::update_posters::*;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn movie_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/movies/all", get(get_all))
        .route("/api/movies", get(search).post(create))
        .route("/api/movies/{id}", 
            get(get_by_id)
                .put(update)
                .delete(delete_movie)  
        )
        .route("/api/movies/latest", get(get_latest))
        .route("/api/movies/genre/{genre}", get(get_by_genre))
        .route("/api/movies/rating/{rating}", get(get_by_rating))
        .route("/api/movies/update-posters", post(update_movie_posters))
}