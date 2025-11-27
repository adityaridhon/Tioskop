use axum::{
    extract::{Path, Query, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySqlPool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;

#[derive(Serialize, FromRow, Clone)]
struct Movie {
    id: i64,
    title: String,
    genre: Option<String>,
    rating: Option<String>,
    duration: Option<i32>,
    description: Option<String>,
    poster_url: Option<String>,
    release_date: Option<chrono::NaiveDate>,
}

#[derive(Serialize, FromRow, Clone)]
struct Showtime {
    id: i64,
    movie_id: Option<i64>,
    studio_id: Option<i64>,
    start_time: Option<chrono::NaiveDateTime>,
    price: Option<rust_decimal::Decimal>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

#[derive(Serialize)]
struct DeleteResponse {
    id: i64,
    deleted: bool,
}

impl<T> ApiResponse<T> {
    fn success(message: &str, data: T) -> Self {
        ApiResponse {
            success: true,
            message: message.to_string(),
            data: Some(data),
        }
    }

    fn error(message: &str) -> Self {
        ApiResponse {
            success: false,
            message: message.to_string(),
            data: None,
        }
    }
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

#[derive(Deserialize)]
struct CreateMovieRequest {
    title: String,
    genre: Option<String>,
    rating: Option<String>,
    duration: Option<i32>,
    description: Option<String>,
    poster_url: Option<String>,
    release_date: Option<chrono::NaiveDate>,
}

#[derive(Deserialize)]
struct UpdateMovieRequest {
    title: Option<String>,
    genre: Option<String>,
    rating: Option<String>,
    duration: Option<i32>,
    description: Option<String>,
    poster_url: Option<String>,
    release_date: Option<chrono::NaiveDate>,
}

#[derive(Deserialize)]
struct CreateShowtimeRequest {
    movie_id: i64,
    studio_id: i64,
    start_time: chrono::NaiveDateTime,
    price: rust_decimal::Decimal,
}

#[derive(Deserialize)]
struct UpdateShowtimeRequest {
    movie_id: Option<i64>,
    studio_id: Option<i64>,
    start_time: Option<chrono::NaiveDateTime>,
    price: Option<rust_decimal::Decimal>,
}

// Ambil semua film 
async fn get_all_movies(
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

// Fungsi mencari film 
async fn search_movies(
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

// Fungsi create movie 
async fn create_movie(
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

// Fungsi update movie by id
async fn update_movie(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Json<ApiResponse<Movie>> {
    // Cek id
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

            let update_result = sqlx::query(
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
            .await;

            match update_result {
                Ok(_) => {
                    sqlx::query_as::<_, Movie>(
                        "SELECT id, title, genre, rating, duration, description, poster_url, release_date FROM movies WHERE id = ?"
                    )
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .map(|movie| Json(ApiResponse::success("Berhasil mengupdate film", movie)))
                    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated movie: {}", e))))
                },
                Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
            }
        },
        Ok(None) => Json(ApiResponse::error(&format!("Film dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Fungsi delete movie by id
async fn delete_movie(
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

// Fungsi get all showtimes
async fn get_all_showtimes(
    State(pool): State<MySqlPool>,
) -> Json<ApiResponse<Vec<Showtime>>> {
    sqlx::query_as::<_, Showtime>(
        "SELECT id, movie_id, studio_id, start_time, price FROM showtimes"
    )
    .fetch_all(&pool)
    .await
    .map(|showtimes| Json(ApiResponse::success("Berhasil mengambil semua showtimes", showtimes)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Fungsi get showtimes by movie_id
async fn get_showtimes_by_movie(
    State(pool): State<MySqlPool>,
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

// Fungsi create showtime 
async fn create_showtime(
    State(pool): State<MySqlPool>,
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

// Fungsi update showtime 
async fn update_showtime(
    State(pool): State<MySqlPool>,
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

            let update_result = sqlx::query(
                "UPDATE showtimes SET movie_id = ?, studio_id = ?, start_time = ?, price = ? WHERE id = ?"
            )
            .bind(updated_movie_id)
            .bind(updated_studio_id)
            .bind(updated_start_time)
            .bind(updated_price)
            .bind(id)
            .execute(&pool)
            .await;

            match update_result {
                Ok(_) => {
                    sqlx::query_as::<_, Showtime>(
                        "SELECT id, movie_id, studio_id, start_time, price FROM showtimes WHERE id = ?"
                    )
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .map(|showtime| Json(ApiResponse::success("Berhasil mengupdate showtime", showtime)))
                    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated showtime: {}", e))))
                },
                Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
            }
        },
        Ok(None) => Json(ApiResponse::error(&format!("Showtime dengan id {} tidak ditemukan", id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Fungsi delete showtime 
async fn delete_showtime(
    State(pool): State<MySqlPool>,
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

#[tokio::main]
async fn main() {
    // env 
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Setup koneksi db
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("Connected to database");

    // Setup CORS unutk FE
    let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any)
        .allow_headers(Any);

    // Build routing endpoint
    let app = Router::new()
        .route("/api/movies/all", get(get_all_movies))
        .route("/api/movies", get(search_movies).post(create_movie))
        .route("/api/movies/{id}", put(update_movie).delete(delete_movie))
        .route("/api/showtimes", get(get_all_showtimes).post(create_showtime))
        .route("/api/showtimes/movie/{movie_id}", get(get_showtimes_by_movie))
        .route("/api/showtimes/{id}", put(update_showtime).delete(delete_showtime))
        .layer(cors)
        .with_state(pool);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(" Server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
