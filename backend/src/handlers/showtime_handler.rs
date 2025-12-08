use crate::models::{showtime::*, response::ApiResponse, DeleteResponse};
use crate::services::showtime;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

fn to_response<T>(result: Result<T, showtime::ShowtimeError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

#[derive(Deserialize)]
pub struct ShowtimeFilterParams {
    pub movie_id: Option<i64>,
    pub studio_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
}

#[derive(Deserialize)]
pub struct DateParam {
    pub date: Option<String>, 
}

/// Get all showtimes
pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(showtime::get_all(&db).await)
}

/// Get showtime by ID
pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Showtime>> {
    to_response(showtime::get_by_id(&db, id).await)
}

/// Get showtimes by movie ID
pub async fn get_by_movie(
    State(db): State<DatabaseConnection>,
    Path(movie_id): Path<i64>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(showtime::get_by_movie(&db, movie_id).await)
}

/// Get showtimes by studio ID
pub async fn get_by_studio(
    State(db): State<DatabaseConnection>,
    Path(studio_id): Path<i64>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(showtime::get_by_studio(&db, studio_id).await)
}

/// Get upcoming showtimes
pub async fn get_upcoming(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(showtime::get_upcoming(&db).await)
}

/// Get today showtimes
pub async fn get_today(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(showtime::get_today(&db).await)
}

/// Get showtimes with filters
pub async fn get_filtered(
    State(db): State<DatabaseConnection>,
    Query(params): Query<ShowtimeFilterParams>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    to_response(
        showtime::get_filtered(&db, params.movie_id, params.studio_id, None, None).await
    )
}

/// Get showtimes by movie and date
pub async fn get_by_movie_and_date(
    State(db): State<DatabaseConnection>,
    Path(movie_id): Path<i64>,
    Query(params): Query<DateParam>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    if let Some(date_str) = params.date {
        match chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            Ok(date) => {
                to_response(showtime::get_by_movie_and_date(&db, movie_id, date).await)
            }
            Err(_) => Json(ApiResponse::error("Format tanggal invalid. Gunakan YYYY-MM-DD")),
        }
    } else {
        Json(ApiResponse::error("Parameter date diperlukan"))
    }
}

/// Create new showtime
pub async fn create(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    showtime::create(&db, payload)
        .await
        .map(|showtime| Json(ApiResponse::success("Berhasil menambahkan showtime", showtime)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}

/// Update showtime
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    showtime::update(&db, id, payload)
        .await
        .map(|showtime| Json(ApiResponse::success("Berhasil mengupdate showtime", showtime)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}

/// Delete showtime
pub async fn delete_showtime(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    showtime::delete(&db, id)
        .await
        .map(|deleted_id| {
            Json(ApiResponse::success(
                "Berhasil menghapus showtime",
                DeleteResponse {
                    id: deleted_id,
                    deleted: true,
                },
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}