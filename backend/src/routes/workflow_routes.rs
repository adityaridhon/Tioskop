use chrono::{DateTime, Local};
use serde::Serialize;
use crate::services::workflow_service::{HasilAnalisa, JadwalWorkflowService, StatusJadwal};
use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse,
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



pub fn workflow_routes() -> Router<AppState> {
    Router::new()
        .route("/jadwal/terdekat", get(get_jadwal_terdekat))
}
