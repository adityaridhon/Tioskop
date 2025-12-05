use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::config::DatabasePools;

pub async fn city_context_middleware(
    State(pools): State<Arc<DatabasePools>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract city from header
    let city = req
        .headers()
        .get("X-City-Name")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_lowercase())
        .unwrap_or_else(|| "balikpapan".to_string()); // default city

    // Get the appropriate pool for the city
    let pool = pools
        .get_city_pool(&city)
        .ok_or_else(|| StatusCode::BAD_REQUEST)?
        .clone();

    // Add pool and city to request extensions
    req.extensions_mut().insert(pool);

    Ok(next.run(req).await)
}
