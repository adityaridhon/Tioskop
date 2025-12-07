use crate::handlers::auth_handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn auth_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/profile", get(get_profile))
}
