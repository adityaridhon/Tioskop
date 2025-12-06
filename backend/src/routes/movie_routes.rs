use crate::handlers::movie_handler::*;
use crate::handlers::update_posters::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::MySqlPool;

pub fn movie_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/movies/all", get(get_all_movies))
        .route("/api/movies", get(search_movies).post(create_movie))
        .route("/api/movies/{id}", put(update_movie).delete(delete_movie))
        .route("/api/movies/update-posters", post(update_movie_posters))
}
