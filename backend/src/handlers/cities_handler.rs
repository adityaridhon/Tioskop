use axum::{extract::State, http::StatusCode, Json};
use std::sync::Arc;
use crate::config::DatabasePools;
use crate::models::{ApiResponse, City, CityInfo};

/// GET /api/cities - Get list of available cities
pub async fn get_cities(
    State(pools): State<Arc<DatabasePools>>,
) -> Result<Json<ApiResponse<Vec<CityInfo>>>, StatusCode> {
    let cities = sqlx::query_as::<_, City>(
        "SELECT id, name, db_name, db_url, is_active FROM cities WHERE is_active = TRUE"
    )
    .fetch_all(pools.get_central())
    .await
    .map_err(|e| {
        eprintln!("❌ Error fetching cities: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let city_infos: Vec<CityInfo> = cities.into_iter().map(|c| c.into()).collect();

    Ok(Json(ApiResponse::success(city_infos)))
}
