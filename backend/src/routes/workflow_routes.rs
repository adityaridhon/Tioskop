// ============================================================================
// WORKFLOW ROUTES - REST API Endpoints untuk Jadwal Workflow
// ============================================================================

use chrono::{DateTime, Local};
use serde::Serialize;

use crate::services::workflow_service::{HasilAnalisa, JadwalWorkflowService, StatusJadwal};
use axum::{
    Json, Router, extract::Path, extract::State, http::StatusCode, response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

// ============================================================================
// RESPONSE TYPES
// ============================================================================

#[derive(Serialize)]
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

// ============================================================================
// APP STATE
// ============================================================================

#[derive(Clone)]
pub struct AppState {
    pub workflow_service: Arc<JadwalWorkflowService>,
}

// ============================================================================
// HANDLER FUNCTIONS
// ============================================================================

/// GET /api/workflow/jadwal/terdekat
/// Endpoint untuk mendapatkan jadwal terdekat
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

/// GET /api/workflow/jadwal/studio/:studio_id
/// Endpoint untuk jadwal terdekat by studio
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

/// GET /api/workflow/jadwal/movie/:movie_id
/// Endpoint untuk jadwal terdekat by movie
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

/// GET /api/workflow/jadwal/stats
/// Endpoint untuk statistik jadwal
pub async fn get_jadwal_stats(State(state): State<AppState>) -> impl IntoResponse {
    use crate::services::workflow_service::JadwalWorkflowService;

    match state.workflow_service.fetch_jadwal_dari_db().await {
        Ok(jadwal_vec) => {
            let jadwal_slice = jadwal_vec.as_slice();

            let total_jadwal = JadwalWorkflowService::count_jadwal(jadwal_slice);
            let jadwal_tertinggi = JadwalWorkflowService::jadwal_harga_tertinggi(jadwal_slice);

            let stats = serde_json::json!({
                "total_jadwal": total_jadwal,
                "jadwal_harga_tertinggi": jadwal_tertinggi.map(|j| j.id),
            });

            (StatusCode::OK, Json(stats))
        }
        Err(e) => {
            let error = serde_json::json!({
                "error": format!("Database error: {}", e)
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

// ============================================================================
// ROUTER CONFIGURATION
// ============================================================================

pub fn workflow_routes() -> Router<AppState> {
    Router::new()
        .route("/jadwal/terdekat", get(get_jadwal_terdekat))
        .route("/jadwal/studio/:studio_id", get(get_jadwal_by_studio))
        .route("/jadwal/movie/:movie_id", get(get_jadwal_by_movie))
        .route("/jadwal/stats", get(get_jadwal_stats))
}

// ============================================================================
// CHECKLIST IMPLEMENTASI: ✅ SELESAI
// ============================================================================
// [x] 1. Implementasi response types (StatusJadwalResponse, ApiResponse)
// [x] 2. Implementasi conversion From<HasilAnalisa> ke ApiResponse
// [x] 3. Implementasi AppState struct
// [x] 4. Implementasi get_jadwal_terdekat handler
// [x] 5. Implementasi get_jadwal_by_studio handler
// [x] 6. Implementasi get_jadwal_by_movie handler
// [x] 7. Implementasi get_jadwal_stats handler
// [x] 8. Setup router dengan workflow_routes()
// [x] 9. Test semua endpoints dengan Postman/curl
// [x] 10. Dokumentasi API di APIDoc.md
// ============================================================================
//
// RESPONSE EXAMPLES:
//
// Success (Mendesak):
// {"status":"Success","result":{"type":"Mendesak","data":{"jadwal_id":1,"waktu_mulai":"2025-12-07T14:30:00+07:00","selisih_menit":15}}}
//
// Success (Aman):
// {"status":"Success","result":{"type":"Aman","data":{"jadwal_id":5,"waktu_mulai":"2025-12-09T22:00:00+07:00"}}}
//
// No Schedule:
// {"status":"NoSchedule"}
//
// Error:
// {"status":"Error","result":{"message":"Database connection failed"}}
// ============================================================================
