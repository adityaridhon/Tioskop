use crate::entities::{Studio, StudiosEntity};
use crate::models::*;
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter};

// Get all studios
pub async fn get_all_studios(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Studio>>> {
    StudiosEntity::find()
        .all(&db)
        .await
        .map(|studios| {
            Json(ApiResponse::success(
                "Berhasil mengambil semua studio",
                studios,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get studio by ID
pub async fn get_studio_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Studio>> {
    StudiosEntity::find_by_id(id)
        .one(&db)
        .await
        .map(|studio_opt| {
            studio_opt
                .map(|studio| Json(ApiResponse::success("Berhasil mengambil studio", studio)))
                .unwrap_or_else(|| Json(ApiResponse::error("Studio tidak ditemukan")))
        })
        .unwrap_or_else(|e| {
            Json(ApiResponse::error(&format!(
                "Database error: {}",
                e
            )))
        })
}

// Get studios by cinema_id
pub async fn get_studios_by_cinema(
    State(db): State<DatabaseConnection>,
    Path(cinema_id): Path<i64>,
) -> Json<ApiResponse<Vec<Studio>>> {
    use crate::entities::studios::Column;

    StudiosEntity::find()
        .filter(Column::CinemaId.eq(cinema_id))
        .all(&db)
        .await
        .map(|studios| {
            Json(ApiResponse::success(
                "Berhasil mengambil studio untuk cinema ini",
                studios,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn create_studio(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    use crate::entities::studios::ActiveModel;

    let new_studio = ActiveModel {
        cinema_id: Set(Some(payload.cinema_id)),
        name: Set(payload.name),
        capacity: Set(payload.capacity),
        r#type: Set(payload.r#type),
        ..Default::default()
    };

    new_studio
        .insert(&db)
        .await
        .map(|studio| Json(ApiResponse::success("Berhasil menambahkan studio", studio)))
        .unwrap_or_else(|e| {
            Json(ApiResponse::error(&format!("Database error: {}", e)))
        })
}

// Update studio
pub async fn update_studio(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    use crate::entities::studios::ActiveModel;

    let studio_exists = StudiosEntity::find_by_id(id).one(&db).await;

    match studio_exists {
        Ok(Some(existing_studio)) => {
            let mut active_studio: ActiveModel = existing_studio.into();

            if let Some(cinema_id) = payload.cinema_id {
                active_studio.cinema_id = Set(Some(cinema_id));
            }
            if let Some(name) = payload.name {
                active_studio.name = Set(name);
            }
            if let Some(capacity) = payload.capacity {
                active_studio.capacity = Set(capacity);
            }
            if payload.r#type.is_some() {
                active_studio.r#type = Set(payload.r#type);
            }

            active_studio
                .update(&db)
                .await
                .map(|studio| Json(ApiResponse::success("Berhasil mengupdate studio", studio)))
                .unwrap_or_else(|e| {
                    Json(ApiResponse::error(&format!(
                        "Failed to update studio: {}",
                        e
                    )))
                })
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Studio dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Delete studio
pub async fn delete_studio(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    use crate::entities::studios::ActiveModel;

    let studio_check = StudiosEntity::find_by_id(id).one(&db).await;

    match studio_check {
        Ok(Some(studio)) => {
            let active_studio: ActiveModel = studio.into();
            active_studio
                .delete(&db)
                .await
                .map(|_| {
                    Json(ApiResponse::success(
                        "Berhasil menghapus studio",
                        DeleteResponse { id, deleted: true },
                    ))
                })
                .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Studio dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}
