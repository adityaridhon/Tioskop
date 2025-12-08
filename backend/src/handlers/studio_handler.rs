use crate::models::{studio::*, response::ApiResponse, DeleteResponse};
use crate::services::studio;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

fn to_response<T>(result: Result<T, studio::StudioError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

#[derive(Deserialize)]
pub struct MinCapacityParams {
    pub min_capacity: Option<i32>,
}

/// Get all studios
pub async fn get_all(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Studio>>> {
    to_response(studio::get_all(&db).await)
}

/// Get studio by ID
pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Studio>> {
    to_response(studio::get_by_id(&db, id).await)
}

/// Get studios by cinema ID
pub async fn get_by_cinema(
    State(db): State<DatabaseConnection>,
    Path(cinema_id): Path<i64>,
) -> Json<ApiResponse<Vec<Studio>>> {
    to_response(studio::get_by_cinema(&db, cinema_id).await)
}

/// Get studios by type
pub async fn get_by_type(
    State(db): State<DatabaseConnection>,
    Path(studio_type): Path<String>,
) -> Json<ApiResponse<Vec<Studio>>> {
    to_response(studio::get_by_type(&db, &studio_type).await)
}

/// Get studios by minimum capacity
pub async fn get_by_min_capacity(
    State(db): State<DatabaseConnection>,
    Query(params): Query<MinCapacityParams>,
) -> Json<ApiResponse<Vec<Studio>>> {
    let min_capacity = params.min_capacity.unwrap_or(0);
    to_response(studio::get_by_min_capacity(&db, min_capacity).await)
}

/// Search studios by name
pub async fn search(
    State(db): State<DatabaseConnection>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<Vec<Studio>>> {
    let query = params.q.unwrap_or_default();
    to_response(studio::search(&db, &query).await)
}

/// Count studios by cinema
pub async fn count_by_cinema(
    State(db): State<DatabaseConnection>,
    Path(cinema_id): Path<i64>,
) -> Json<ApiResponse<u64>> {
    to_response(studio::count_by_cinema(&db, cinema_id).await)
}

/// Count total studios
pub async fn count_total(State(db): State<DatabaseConnection>) -> Json<ApiResponse<u64>> {
    to_response(studio::count_total(&db).await)
}

/// Create new studio
pub async fn create(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    studio::create(&db, payload)
        .await
        .map(|studio| Json(ApiResponse::success("Berhasil menambahkan studio", studio)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}

/// Update studio
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    studio::update(&db, id, payload)
        .await
        .map(|studio| Json(ApiResponse::success("Berhasil mengupdate studio", studio)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}

/// Delete studio
pub async fn delete_studio(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    studio::delete(&db, id)
        .await
        .map(|deleted_id| {
            Json(ApiResponse::success(
                "Berhasil menghapus studio",
                DeleteResponse {
                    id: deleted_id,
                    deleted: true,
                },
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}