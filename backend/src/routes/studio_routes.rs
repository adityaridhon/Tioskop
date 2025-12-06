use crate::handlers::studio_handler::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sqlx::MySqlPool;

pub fn studio_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/studios", get(get_all_studios).post(create_studio))
        .route(
            "/api/studios/{id}",
            get(get_studio_by_id)
                .put(update_studio)
                .delete(delete_studio),
        )
        .route(
            "/api/studios/cinema/{cinema_id}",
            get(get_studios_by_cinema),
        )
}
