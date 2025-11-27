use axum::{extract::{Path, Query, State}, Json};
use sqlx::MySqlPool;
use crate::models::*;

pub async fn get_all_movies(
    State(pool): State<MySqlPool>,
) -> Json<ApiResponse<Vec<Movie>>> {
    sqlx::query_as::<_, Movie>(
        "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies"
    )
    .fetch_all(&pool)
    .await
    .map(|movies| Json(ApiResponse::success("Berhasil mengambil semua film", movies)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn search_movies(
    State(pool): State<MySqlPool>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<Vec<Movie>>> {
    let search_pattern = params.q
        .map(|query_str| format!("%{}%", query_str))
        .unwrap_or_else(|| "%".to_string());

    sqlx::query_as::<_, Movie>(
        "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE title LIKE ?"
    )
    .bind(search_pattern)
    .fetch_all(&pool)
    .await
    .map(|movies| Json(ApiResponse::success("Berhasil mencari film", movies)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

pub async fn create_movie(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateMovieRequest>,
) -> Json<ApiResponse<Movie>> {
    let insert_result = sqlx::query(
        "INSERT INTO movies (title, genre, rating, duration, description, poster_url, release_date) VALUES (?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&payload.title)
    .bind(&payload.genre)
    .bind(&payload.rating)
    .bind(payload.duration)
    .bind(&payload.description)
    .bind(&payload.poster_url)
    .bind(payload.release_date)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(result) => {
            let movie_id = result.last_insert_id() as i64;
            
            sqlx::query_as::<_, Movie>(
                "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE id = ?"
            )
            .bind(movie_id)
            .fetch_one(&pool)
            .await
            .map(|movie| Json(ApiResponse::success("Berhasil menambahkan film", movie)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch created movie: {}", e))))
        },
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

pub async fn update_movie(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Json<ApiResponse<Movie>> {
    let movie_exists = sqlx::query_as::<_, Movie>(
        "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match movie_exists {
        Ok(Some(existing_movie)) => {
            let updated_title = payload.title.unwrap_or(existing_movie.title);
            let updated_genre = payload.genre.or(existing_movie.genre);
            let updated_rating = payload.rating.or(existing_movie.rating);
            let updated_duration = payload.duration.or(existing_movie.duration);
            let updated_description = payload.description.or(existing_movie.description);
            let updated_poster_url = payload.poster_url.or(existing_movie.poster_url);
            let updated_release_date = payload.release_date.or(existing_movie.release_date);

            sqlx::query(
                "UPDATE movies SET title = ?, genre = ?, rating = ?, duration = ?, description = ?, poster_url = ?, release_date = ? WHERE id = ?"
            )
            .bind(&updated_title)
            .bind(&updated_genre)
            .bind(&updated_rating)
            .bind(updated_duration)
            .bind(&updated_description)
            .bind(&updated_poster_url)
            .bind(updated_release_date)
            .bind(id)
            .execute(&pool)
            .await
            .ok();

            sqlx::query_as::<_, Movie>(
                "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE id = ?"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .map(|movie| Json(ApiResponse::success("Berhasil mengupdate film", movie)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated movie: {}", e))))
        },
        Ok(None) => Json(ApiResponse::error(&format!("Film dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

pub async fn delete_movie(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    let movie_check = sqlx::query_as::<_, Movie>(
        "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match movie_check {
        Ok(Some(_)) => {
            sqlx::query("DELETE FROM movies WHERE id = ?")
                .bind(id)
                .execute(&pool)
                .await
                .map(|_| Json(ApiResponse::success(
                    "Berhasil menghapus film",
                    DeleteResponse { id, deleted: true }
                )))
                .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to delete movie: {}", e))))
        },
        Ok(None) => Json(ApiResponse::error(&format!("Film dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}