use crate::handlers::studio_handler;
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use sea_orm::DatabaseConnection;

pub fn studio_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route(
            "/api/studios",
            get(studio_handler::get_all).post(studio_handler::create),
        )
        .route(
            "/api/studios/{id}",
            get(studio_handler::get_by_id)
                .put(studio_handler::update)
                .delete(studio_handler::delete_studio),
        )
        
        // Query endpoints
        .route(
            "/api/studios/cinema/{cinema_id}",
            get(studio_handler::get_by_cinema),
        )
        .route(
            "/api/studios/type/{studio_type}",
            get(studio_handler::get_by_type),
        )
        .route(
            "/api/studios/capacity",
            get(studio_handler::get_by_min_capacity),
        )
        .route("/api/studios/search", get(studio_handler::search))
        .route(
            "/api/studios/cinema/{cinema_id}/count",
            get(studio_handler::count_by_cinema),
        )
        .route("/api/studios/count", get(studio_handler::count_total))
}