use crate::entities::{Movie, MoviesEntity};
use crate::models::*;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, Set, QueryOrder, QuerySelect};

pub async fn get_all_movies(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Movie>>> {
    MoviesEntity::find()
        .all(&db)
        .await
        .map(|movies| Json(ApiResponse::success("Berhasil mengambil semua film", movies)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn search_movies(
    State(db): State<DatabaseConnection>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<Vec<Movie>>> {
    use crate::entities::movies::Column;

    let search_pattern = params
        .q
        .map(|q| format!("%{}%", q))
        .unwrap_or_else(|| "%".into());

    MoviesEntity::find()
        .filter(Column::Title.like(&search_pattern))
        .order_by_desc(Column::ReleaseDate)
        .limit(15)
        .all(&db)
        .await
        .map(|movies| Json(ApiResponse::success("OK", movies)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}

pub async fn create_movie(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateMovieRequest>,
) -> Json<ApiResponse<Movie>> {
    use crate::entities::movies::ActiveModel;

    let new_movie = ActiveModel {
        title: Set(payload.title),
        genre: Set(payload.genre),
        rating: Set(payload.rating),
        duration: Set(payload.duration),
        description: Set(payload.description),
        poster_url: Set(payload.poster_url),
        release_date: Set(payload.release_date),
        ..Default::default()
    };

    new_movie
        .insert(&db)
        .await
        .map(|movie| Json(ApiResponse::success("Berhasil menambahkan film", movie)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn update_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Json<ApiResponse<Movie>> {
    use crate::entities::movies::ActiveModel;

    let movie_exists = MoviesEntity::find_by_id(id).one(&db).await;

    match movie_exists {
        Ok(Some(existing_movie)) => {
            let mut active_movie: ActiveModel = existing_movie.into();

            if let Some(title) = payload.title {
                active_movie.title = Set(title);
            }
            if payload.genre.is_some() {
                active_movie.genre = Set(payload.genre);
            }
            if payload.rating.is_some() {
                active_movie.rating = Set(payload.rating);
            }
            if payload.duration.is_some() {
                active_movie.duration = Set(payload.duration);
            }
            if payload.description.is_some() {
                active_movie.description = Set(payload.description);
            }
            if payload.poster_url.is_some() {
                active_movie.poster_url = Set(payload.poster_url);
            }
            if payload.release_date.is_some() {
                active_movie.release_date = Set(payload.release_date);
            }

            active_movie
                .update(&db)
                .await
                .map(|movie| Json(ApiResponse::success("Berhasil mengupdate film", movie)))
                .unwrap_or_else(|e| {
                    Json(ApiResponse::error(&format!(
                        "Failed to update movie: {}",
                        e
                    )))
                })
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Film dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

pub async fn delete_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    use crate::entities::movies::ActiveModel;

    let movie_check = MoviesEntity::find_by_id(id).one(&db).await;

    match movie_check {
        Ok(Some(movie)) => {
            let active_movie: ActiveModel = movie.into();
            active_movie
                .delete(&db)
                .await
                .map(|_| {
                    Json(ApiResponse::success(
                        "Berhasil menghapus film",
                        DeleteResponse { id, deleted: true },
                    ))
                })
                .unwrap_or_else(|e| {
                    Json(ApiResponse::error(&format!(
                        "Failed to delete movie: {}",
                        e
                    )))
                })
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Film dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}
