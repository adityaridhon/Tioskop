use axum::{
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use crate::config::DatabasePools;
use crate::handlers::auth_handler::*;

pub fn auth_routes() -> Router<Arc<DatabasePools>> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/profile", get(get_profile))
}
