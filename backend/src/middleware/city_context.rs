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

    println!("🏙️  Request for city: {}", city);

    // Get the appropriate pool
    let pool = pools
        .get_city_pool(&city)
        .ok_or_else(|| {
            eprintln!("❌ Invalid city: {}", city);
            StatusCode::BAD_REQUEST
        })?
        .clone();

    // Add to request extensions
    req.extensions_mut().insert(pool);
    req.extensions_mut().insert(city);

    Ok(next.run(req).await)
}
