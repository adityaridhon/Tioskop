use axum::{extract::{Path, Extension}, Json};
use sqlx::MySqlPool;
use crate::models::*;

pub async fn get_all_showtimes(
    Extension(pool): Extension<MySqlPool>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    sqlx::query_as::<_, Showtime>(
        "SELECT id, movie_id, studio_id, start_time, price FROM showtimes"
    )
    .fetch_all(&pool)
    .await
    .map(|showtimes| Json(ApiResponse::success("Berhasil mengambil semua showtimes", showtimes)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn get_showtimes_by_movie(
    Extension(pool): Extension<MySqlPool>,
    Path(movie_id): Path<i64>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    sqlx::query_as::<_, Showtime>(
        "SELECT id, movie_id, studio_id, start_time, price FROM showtimes WHERE movie_id = ?"
    )
    .bind(movie_id)
    .fetch_all(&pool)
    .await
    .map(|showtimes| Json(ApiResponse::success("Berhasil mengambil showtimes untuk film ini", showtimes)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn create_showtime(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<CreateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    let insert_result = sqlx::query(
        "INSERT INTO showtimes (movie_id, studio_id, start_time, price) VALUES (?, ?, ?, ?)"
    )
    .bind(payload.movie_id)
    .bind(payload.studio_id)
    .bind(payload.start_time)
    .bind(payload.price)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(result) => {
            let showtime_id = result.last_insert_id() as i64;
            
            sqlx::query_as::<_, Showtime>(
                "SELECT id, movie_id, studio_id, start_time, price FROM showtimes WHERE id = ?"
            )
            .bind(showtime_id)
            .fetch_one(&pool)
            .await
            .map(|showtime| Json(ApiResponse::success("Berhasil menambahkan showtime", showtime)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch created showtime: {}", e))))
        },
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

pub async fn update_showtime(
    Extension(pool): Extension<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    let showtime_exists = sqlx::query_as::<_, Showtime>(
        "SELECT id, movie_id, studio_id, start_time, price FROM showtimes WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match showtime_exists {
        Ok(Some(existing_showtime)) => {
            let updated_movie_id = payload.movie_id.or(existing_showtime.movie_id);
            let updated_studio_id = payload.studio_id.or(existing_showtime.studio_id);
            let updated_start_time = payload.start_time.or(existing_showtime.start_time);
            let updated_price = payload.price.or(existing_showtime.price);

            sqlx::query(
                "UPDATE showtimes SET movie_id = ?, studio_id = ?, start_time = ?, price = ? WHERE id = ?"
            )
            .bind(updated_movie_id)
            .bind(updated_studio_id)
            .bind(updated_start_time)
            .bind(updated_price)
            .bind(id)
            .execute(&pool)
            .await
            .ok();

            sqlx::query_as::<_, Showtime>(
                "SELECT id, movie_id, studio_id, start_time, price FROM showtimes WHERE id = ?"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .map(|showtime| Json(ApiResponse::success("Berhasil mengupdate showtime", showtime)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated showtime: {}", e))))
        },
        Ok(None) => Json(ApiResponse::error(&format!("Showtime dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

pub async fn delete_showtime(
    Extension(pool): Extension<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    sqlx::query("DELETE FROM showtimes WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map(|result| {
            let deleted = result.rows_affected() > 0;
            if deleted {
                Json(ApiResponse::success("Berhasil menghapus showtime", DeleteResponse { id, deleted }))
            } else {
                Json(ApiResponse::error(&format!("Showtime dengan id {} tidak ditemukan", id)))
            }
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}
