use axum::{
    routing::post,
    Router,
};
use sqlx::MySqlPool;
use crate::handlers::auth_handler::*;

pub fn auth_routes() -> Router<MySqlPool> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
}
