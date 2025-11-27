use axum::{routing::{get, post, put, delete}, Router};
use sqlx::MySqlPool;
use crate::handlers::studio_handler::*;

pub fn studio_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/studios", get(get_all_studios).post(create_studio))
        .route("/api/studios/{id}", get(get_studio_by_id).put(update_studio).delete(delete_studio))
        .route("/api/studios/cinema/{cinema_id}", get(get_studios_by_cinema))
}
