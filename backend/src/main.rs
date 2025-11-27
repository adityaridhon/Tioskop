mod config;
mod models;
mod handlers;
mod routes;

use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use routes::{movie_routes::movie_routes, showtime_routes::showtime_routes};

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Setup database
    let pool = config::create_pool().await;
    println!("Connected to database");

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build dengan routes
    let app = Router::new()
        .merge(movie_routes())
        .merge(showtime_routes())
        .layer(cors)
        .with_state(pool);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(" Server on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}