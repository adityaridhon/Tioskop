use crate::handlers::movie_handler::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
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
}