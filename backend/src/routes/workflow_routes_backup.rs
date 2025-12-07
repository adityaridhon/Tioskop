// ============================================================================
// WORKFLOW ROUTES - API Endpoints untuk Jadwal Workflow
// ============================================================================

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::services::workflow_service::{HasilAnalisa, JadwalWorkflowService, StatusJadwal};
use axum::{
    Json, Router, extract::Path, extract::State, http::StatusCode, response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

#[derive(Clone)]
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

// Convert internal enum ke response enum
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

// ============================================================================
    pub workflow_service: Arc<JadwalWorkflowService>,
}

/// GET /api/workflow/jadwal/terdekatinto();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

/// GET /api/workflow/jadwal/studio/:studio_id
/// Endpoint untuk jadwal terdekat by studio
pub async fn get_jadwal_by_studio(
    State(state): State<AppState>,
    Path(studio_id): Path<i64>,
) -> impl IntoResponse {
    match state
}

/// GET /api/workflow/jadwal/studio/:studio_id
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

/// GET /api/workflow/jadwal/movie/:movie_id
/// Endpoint untuk jadwal terdekat by movie
pub async fn get_jadwal_by_movie(
    State(state): State<AppState>,
}

/// GET /api/workflow/jadwal/movie/:movie_id
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

/// GET /api/workflow/jadwal/stats
/// 🔥 MULTIPROCESSING: Endpoint untuk statistik jadwal dengan parallel aggregation
pub async fn get_jadwal_stats(State(state): State<AppState>) -> impl IntoResponse {
    match state.workflow_service.execute_workflow_statistik().await {
        Ok(statistik) => {
}

/// 🔥 RAYON: GET /api/workflow/jadwal/stats - Parallel aggregation
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

/// GET /api/workflow/jadwal/batch?chunk_size=100
/// 🔥 MULTIPROCESSING: Batch processing dengan chunk-based parallel execution
pub async fn get_jadwal_batch(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
    }
}

/// 🔥 RAYON: GET /api/workflow/jadwal/batch - Chunk-based parallel processing

    match state.workflow_service.execute_workflow_batch(chunk_size).await {
        Ok(hasil_batch) => {
            let results: Vec<_> = hasil_batch
                .into_iter()
                .map(|hasil| {
                    serde_json::json!({
                        "type": match hasil {
                            crate::services::workflow_service::HasilAnalisa::JadwalTerdekat(_) => "found",
                            crate::services::workflow_service::HasilAnalisa::TidakAdaJadwal => "not_found",
                            crate::services::workflow_service::HasilAnalisa::Error(_) => "error",
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

/// POST /api/workflow/jadwal/filter-kompleks
    }
}

/// 🔥 RAYON: POST /api/workflow/jadwal/filter-kompleks - Multi-level parallel filtering
    use crate::services::workflow_service::FilterKriteria;

    let kriteria = FilterKriteria {
        studio_id: body.get("studio_id").and_then(|v| v.as_i64()),
        movie_id: body.get("movie_id").and_then(|v| v.as_i64()),
        min_harga: body.get("min_harga").and_then(|v| v.as_i64()),
        max_harga: body.get("max_harga").and_then(|v| v.as_i64()),
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

/// GET /api/workflow/jadwal/film/:movie_id/semua-bioskop
/// 🔥 MULTIPROCESSING: Jadwal terdekat untuk 1 film di semua bioskop
    }
}

/// 🔥 RAYON: GET /api/workflow/jadwal/film/:movie_id/semua-bioskop
/// Jadwal terdekat untuk 1 film di semua bioskop dengan parallel grouping
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

/// GET /api/workflow/jadwal/semua-film
/// 🔥 MULTIPROCESSING: Jadwal terdekat untuk semua film
    }
}

/// 🔥 RAYON: GET /api/workflow/jadwal/semua-film
/// Jadwal terdekat untuk semua film dengan parallel grouping
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

// ============================================================================
    }
}

pub fn workflow_routes() -> Router<AppState> {(get_jadwal_by_movie))
        .route("/jadwal/stats", get(get_jadwal_stats))
        .route("/jadwal/batch", get(get_jadwal_batch))
        .route("/jadwal/filter-kompleks", axum::routing::post(post_jadwal_filter_kompleks))
        .route("/jadwal/film/{movie_id}/semua-bioskop", get(get_jadwal_film_semua_bioskop))  // 🔥 NEW
        .route("/jadwal/semua-film", get(get_jadwal_semua_film))  // 🔥 NEW
}

// ============================================================================
// 🔥 MULTIPROCESSING IMPLEMENTATION CHECKLIST: ✅ SELESAI
        .route("/jadwal/batch", get(get_jadwal_batch))
        .route("/jadwal/filter-kompleks", axum::routing::post(post_jadwal_filter_kompleks))
        .route("/jadwal/film/{movie_id}/semua-bioskop", get(get_jadwal_film_semua_bioskop))
        .route("/jadwal/semua-film", get(get_jadwal_semua_film))
}