use crate::models::{movie::*, response::ApiResponse, DeleteResponse};
use crate::services::movie;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::DatabaseConnection;

fn to_response<T>(result: Result<T, movie::MovieError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

/// Get all movies
pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_all(&db).await)
}

/// Search movies
pub async fn search(
    State(db): State<DatabaseConnection>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::search(&db, params.q).await)
}

/// Get movie by ID
pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::get_by_id(&db, id).await)
}

pub async fn create(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateMovieRequest>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::create(&db, payload).await)
}

/// Update movie
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::update(&db, id, payload).await)
}

/// Delete movie
pub async fn delete_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    movie::delete(&db, id)
        .await
        .map(|id| {
            Json(ApiResponse::success(
                "Berhasil menghapus film",
                DeleteResponse { id, deleted: true },
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}
/// Get movies by genre
pub async fn get_by_genre(
    State(db): State<DatabaseConnection>,
    Path(genre): Path<String>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_by_genre(&db, &genre).await)
}

/// Get movies by rating
pub async fn get_by_rating(
    State(db): State<DatabaseConnection>,
    Path(rating): Path<String>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_by_rating(&db, &rating).await)
}

/// Get latest movies
pub async fn get_latest(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_latest(&db, 10).await)
}