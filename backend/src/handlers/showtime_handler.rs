use crate::entities::{Showtime, ShowtimesEntity};
use crate::models::*;
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait};

pub async fn get_all_showtimes(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Showtime>>> {
    ShowtimesEntity::find()
        .all(&db)
        .await
        .map(|showtimes| {
            Json(ApiResponse::success(
                "Berhasil mengambil semua showtimes",
                showtimes,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn get_showtimes_by_movie(
    State(db): State<DatabaseConnection>,
    Path(movie_id): Path<i64>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    use crate::entities::showtimes::Column;

    ShowtimesEntity::find()
        .filter(Column::MovieId.eq(movie_id))
        .all(&db)
        .await
        .map(|showtimes| {
            Json(ApiResponse::success(
                "Berhasil mengambil showtimes untuk film ini",
                showtimes,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn create_showtime(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    use crate::entities::showtimes::ActiveModel;

    let new_showtime = ActiveModel {
        movie_id: Set(Some(payload.global_movie_id)),
        studio_id: Set(Some(payload.studio_id)),
        start_time: Set(Some(payload.start_time.and_local_timezone(chrono::Local).unwrap())),
        price: Set(Some(payload.price)),
        ..Default::default()
    };

    new_showtime
        .insert(&db)
        .await
        .map(|showtime| {
            Json(ApiResponse::success(
                "Berhasil menambahkan showtime",
                showtime,
            ))
        })
        .unwrap_or_else(|e| {
            Json(ApiResponse::error(&format!(
                "Database error: {}",
                e
            )))
        })
}

pub async fn update_showtime(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateShowtimeRequest>,
) -> Json<ApiResponse<Showtime>> {
    use crate::entities::showtimes::ActiveModel;

    let showtime_exists = ShowtimesEntity::find_by_id(id).one(&db).await;

    match showtime_exists {
        Ok(Some(existing)) => {
            let mut showtime: ActiveModel = existing.into();

            if let Some(movie_id) = payload.global_movie_id {
                showtime.movie_id = Set(Some(movie_id));
            }
            if let Some(studio_id) = payload.studio_id {
                showtime.studio_id = Set(Some(studio_id));
            }
            if let Some(start_time) = payload.start_time {
                showtime.start_time = Set(Some(start_time.and_local_timezone(chrono::Local).unwrap()));
            }
            if let Some(price) = payload.price {
                showtime.price = Set(Some(price));
            }

            showtime
                .update(&db)
                .await
                .map(|updated| {
                    Json(ApiResponse::success("Berhasil mengupdate showtime", updated))
                })
                .unwrap_or_else(|e| {
                    Json(ApiResponse::error(&format!("Database error: {}", e)))
                })
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Showtime dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

pub async fn delete_showtime(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    use crate::entities::showtimes::ActiveModel;
    use sea_orm::ActiveValue;

    ShowtimesEntity::delete_by_id(id)
        .exec(&db)
        .await
        .map(|result| {
            Json(ApiResponse::success(
                "Berhasil menghapus showtime",
                DeleteResponse {
                    id,
                    deleted: result.rows_affected > 0,
                },
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}