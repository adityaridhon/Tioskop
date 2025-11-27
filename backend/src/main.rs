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
    // Cek id
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
        .layer(cors)
        .with_state(pool);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(" Server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
