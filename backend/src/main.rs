use axum::{
    extract::{Query, State},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySqlPool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;

#[derive(Serialize, FromRow)]
struct Movie {
    id: i32,
    title: String,
    description: Option<String>,
    poster_url: Option<String>,
    rating: Option<f32>,
    year: Option<i32>,
}

#[derive(Deserialize)]
struct SearchParams {
    q: Option<String>,
}

async fn search_movies(
    State(pool): State<MySqlPool>,
    Query(params): Query<SearchParams>,
) -> Result<Json<Vec<Movie>>, String> {
    let query_str = params.q.unwrap_or_default();
    let search_pattern = format!("%{}%", query_str);

    // Query database for movies matching the title
    // Assumes a table named 'movies' exists
    let movies = sqlx::query_as::<_, Movie>(
        "SELECT id, title, description, poster_url, rating, year FROM movies WHERE title LIKE ?"
    )
    .bind(search_pattern)
    .fetch_all(&pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(Json(movies))
}

#[tokio::main]
async fn main() {
    // Load environment variables from .env
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Setup database connection pool
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("✅ Connected to database");

    // Setup CORS to allow frontend requests
    let cors = CorsLayer::new()
        .allow_origin(Any) // For development, allow any origin. In prod, specify frontend URL.
        .allow_methods(Any)
        .allow_headers(Any);

    // Build our application with a route
    let app = Router::new()
        .route("/api/movies", get(search_movies))
        .layer(cors)
        .with_state(pool);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
