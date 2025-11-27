use axum::{extract::{Path, State}, Json};
use sqlx::MySqlPool;
use crate::models::*;

// Get all studios 
pub async fn get_all_studios(
    State(pool): State<MySqlPool>,
) -> Json<ApiResponse<Vec<Studio>>> {
    sqlx::query_as::<_, Studio>(
        "SELECT id, cinema_id, name, capacity, type FROM studios"
    )
    .fetch_all(&pool)
    .await
    .map(|studios| Json(ApiResponse::success("Berhasil mengambil semua studio", studios)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get studio by ID
pub async fn get_studio_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Studio>> {
    sqlx::query_as::<_, Studio>(
        "SELECT id, cinema_id, name, capacity, type FROM studios WHERE id = ?"
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map(|studio| Json(ApiResponse::success("Berhasil mengambil studio", studio)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Studio tidak ditemukan: {}", e))))
}

// Get studios by cinema_id
pub async fn get_studios_by_cinema(
    State(pool): State<MySqlPool>,
    Path(cinema_id): Path<i64>,
) -> Json<ApiResponse<Vec<Studio>>> {
    sqlx::query_as::<_, Studio>(
        "SELECT id, cinema_id, name, capacity, type FROM studios WHERE cinema_id = ?"
    )
    .bind(cinema_id)
    .fetch_all(&pool)
    .await
    .map(|studios| Json(ApiResponse::success("Berhasil mengambil studio untuk cinema ini", studios)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Create studio
pub async fn create_studio(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    let insert_result = sqlx::query(
        "INSERT INTO studios (cinema_id, name, capacity, type) VALUES (?, ?, ?, ?)"
    )
    .bind(payload.cinema_id)
    .bind(&payload.name)
    .bind(payload.capacity)
    .bind(&payload.r#type)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(result) => {
            let studio_id = result.last_insert_id() as i64;
            
            sqlx::query_as::<_, Studio>(
                "SELECT id, cinema_id, name, capacity, type FROM studios WHERE id = ?"
            )
            .bind(studio_id)
            .fetch_one(&pool)
            .await
            .map(|studio| Json(ApiResponse::success("Berhasil menambahkan studio", studio)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch created studio: {}", e))))
        },
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Update studio
pub async fn update_studio(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateStudioRequest>,
) -> Json<ApiResponse<Studio>> {
    let studio_exists = sqlx::query_as::<_, Studio>(
        "SELECT id, cinema_id, name, capacity, type FROM studios WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match studio_exists {
        Ok(Some(existing_studio)) => {
            let updated_cinema_id = payload.cinema_id.or(existing_studio.cinema_id);
            let updated_name = payload.name.unwrap_or(existing_studio.name);
            let updated_capacity = payload.capacity.unwrap_or(existing_studio.capacity);
            let updated_type = payload.r#type.or(existing_studio.r#type);

            sqlx::query(
                "UPDATE studios SET cinema_id = ?, name = ?, capacity = ?, type = ? WHERE id = ?"
            )
            .bind(updated_cinema_id)
            .bind(&updated_name)
            .bind(updated_capacity)
            .bind(&updated_type)
            .bind(id)
            .execute(&pool)
            .await
            .ok();

            sqlx::query_as::<_, Studio>(
                "SELECT id, cinema_id, name, capacity, type FROM studios WHERE id = ?"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .map(|studio| Json(ApiResponse::success("Berhasil mengupdate studio", studio)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated studio: {}", e))))
        },
        Ok(None) => Json(ApiResponse::error(&format!("Studio dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Delete studio
pub async fn delete_studio(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    sqlx::query("DELETE FROM studios WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map(|result| {
            let deleted = result.rows_affected() > 0;
            if deleted {
                Json(ApiResponse::success("Berhasil menghapus studio", DeleteResponse { id, deleted }))
            } else {
                Json(ApiResponse::error(&format!("Studio dengan id {} tidak ditemukan", id)))
            }
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}
