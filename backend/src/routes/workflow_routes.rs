use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::Serialize;
use crate::services::workflow_service::{HasilAnalisa, JadwalWorkflowService, StatusJadwal};
use axum::{
    Json, Router, extract::Path, extract::State, http::StatusCode, response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

#[derive(Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum StatusJadwalResponse {
    Mendesak {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
        selisih_menit: i64,
    },
    Aman {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
    },
    Selesai {
        jadwal_id: i64,
        waktu_selesai: DateTime<Local>,
    },
}

#[derive(Serialize)]
#[serde(tag = "status", content = "result")]
pub enum ApiResponse {
    Success(StatusJadwalResponse),
    NoSchedule,
    Error { message: String },
}

impl From<StatusJadwal> for StatusJadwalResponse {
    fn from(status: StatusJadwal) -> Self {
        match status {
            StatusJadwal::Mendesak {
                jadwal_id,
                waktu_mulai,
                selisih_menit,
            } => StatusJadwalResponse::Mendesak {
                jadwal_id,
                waktu_mulai,
                selisih_menit,
            },
            StatusJadwal::Aman {
                jadwal_id,
                waktu_mulai,
            } => StatusJadwalResponse::Aman {
                jadwal_id,
                waktu_mulai,
            },
            StatusJadwal::Selesai {
                jadwal_id,
                waktu_selesai,
            } => StatusJadwalResponse::Selesai {
                jadwal_id,
                waktu_selesai,
            },
        }
    }
}

impl From<HasilAnalisa> for ApiResponse {
    fn from(hasil: HasilAnalisa) -> Self {
        match hasil {
            HasilAnalisa::JadwalTerdekat(status) => ApiResponse::Success(status.into()),
            HasilAnalisa::TidakAdaJadwal => ApiResponse::NoSchedule,
            HasilAnalisa::Error(msg) => ApiResponse::Error { message: msg },
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub workflow_service: Arc<JadwalWorkflowService>,
}

pub async fn get_jadwal_terdekat(State(state): State<AppState>) -> impl IntoResponse {
    match state.workflow_service.execute_workflow().await {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_by_studio(
    State(state): State<AppState>,
    Path(studio_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_by_studio(studio_id)
        .await
    {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_by_movie(
    State(state): State<AppState>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_by_movie(movie_id)
        .await
    {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_stats(State(state): State<AppState>) -> impl IntoResponse {
    match state.workflow_service.execute_workflow_statistik().await {
        Ok(statistik) => {
            let response = serde_json::json!({
                "success": true,
                "statistik": {
                    "total_jadwal": statistik.total_jadwal,
                    "jadwal_hari_ini": statistik.jadwal_hari_ini,
                    "jadwal_minggu_ini": statistik.jadwal_minggu_ini,
                    "jadwal_mendesak": statistik.jadwal_mendesak,
                    "harga_rata_rata": statistik.harga_rata_rata,
                    "harga_tertinggi": statistik.harga_tertinggi,
                    "harga_terendah": statistik.harga_terendah,
                    "studio_terpopuler": statistik.studio_terpopuler,
                    "movie_terpopuler": statistik.movie_terpopuler,
                }
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn get_jadwal_batch(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let chunk_size = params
        .get("chunk_size")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);

    match state.workflow_service.execute_workflow_batch(chunk_size).await {
        Ok(hasil_batch) => {
            let results: Vec<_> = hasil_batch
                .into_iter()
                .map(|hasil| {
                    serde_json::json!({
                        "type": match hasil {
                            HasilAnalisa::JadwalTerdekat(_) => "found",
                            HasilAnalisa::TidakAdaJadwal => "not_found",
                            HasilAnalisa::Error(_) => "error",
                        }
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "chunk_size": chunk_size,
                "total_chunks": results.len(),
                "results": results,
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn post_jadwal_filter_kompleks(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    use crate::services::workflow_service::FilterKriteria;

    let kriteria = FilterKriteria {
        studio_id: body.get("studio_id").and_then(|v| v.as_i64()),
        movie_id: body.get("movie_id").and_then(|v| v.as_i64()),
        min_harga: body.get("min_harga").and_then(|v| v.as_i64()).map(|i| Decimal::from(i)),
        max_harga: body.get("max_harga").and_then(|v| v.as_i64()).map(|i| Decimal::from(i)),
        hari_ini_saja: body.get("hari_ini_saja").and_then(|v| v.as_bool()).unwrap_or(false),
        hanya_mendesak: body.get("hanya_mendesak").and_then(|v| v.as_bool()).unwrap_or(false),
    };

    match state.workflow_service.execute_workflow_kompleks(kriteria).await {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_film_semua_bioskop(
    State(state): State<AppState>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_jadwal_film_semua_bioskop(movie_id)
        .await
    {
        Ok(hasil_list) => {
            let results: Vec<_> = hasil_list
                .into_iter()
                .map(|(studio_id, status)| {
                    let response: StatusJadwalResponse = status.into();
                    serde_json::json!({
                        "studio_id": studio_id,
                        "jadwal": response
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "movie_id": movie_id,
                "total_bioskop": results.len(),
                "jadwal_per_bioskop": results
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn get_jadwal_semua_film(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_jadwal_semua_film()
        .await
    {
        Ok(hasil_list) => {
            let results: Vec<_> = hasil_list
                .into_iter()
                .map(|(movie_id, status)| {
                    let response: StatusJadwalResponse = status.into();
                    serde_json::json!({
                        "movie_id": movie_id,
                        "jadwal": response
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "total_film": results.len(),
                "jadwal_per_film": results
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub fn workflow_routes() -> Router<AppState> {
    Router::new()
        .route("/jadwal/terdekat", get(get_jadwal_terdekat))
        .route("/jadwal/studio/{studio_id}", get(get_jadwal_by_studio))
        .route("/jadwal/movie/{movie_id}", get(get_jadwal_by_movie))
        .route("/jadwal/stats", get(get_jadwal_stats))
        .route("/jadwal/batch", get(get_jadwal_batch))
        .route("/jadwal/filter-kompleks", axum::routing::post(post_jadwal_filter_kompleks))
        .route("/jadwal/film/{movie_id}/semua-bioskop", get(get_jadwal_film_semua_bioskop))
        .route("/jadwal/semua-film", get(get_jadwal_semua_film))
}
