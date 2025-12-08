mod entities;
mod handlers;
mod middleware;
mod models;
mod routes;
mod services;

use axum::Router;
use dotenvy::dotenv;
use routes::workflow_routes::AppState as WorkflowAppState;
use routes::{
    auth_routes::auth_routes, booking_routes::booking_routes, movie_routes::movie_routes,
    seat_routes::seat_routes, showtime_routes::showtime_routes, studio_routes::studio_routes,
    workflow_routes,
};
use sea_orm::Database;
use services::workflow_service::JadwalWorkflowService;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let db_connection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");

    println!("Connected to database with SeaORM");

    let workflow_service = Arc::new(JadwalWorkflowService::new(db_connection.clone()));

    let workflow_state = WorkflowAppState {
        workflow_service: workflow_service.clone(),
    };

    println!("Workflow service initialized");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app_routes = Router::new()
        .merge(auth_routes())
        .merge(movie_routes())
        .merge(showtime_routes())
        .merge(studio_routes())
        .merge(seat_routes())
        .merge(booking_routes())
        .with_state(db_connection);

    let workflow_router = Router::new()
        .nest("/api/workflow", workflow_routes::workflow_routes())
        .with_state(workflow_state);

    let app = Router::new()
        .merge(app_routes)
        .merge(workflow_router)
        .layer(cors);

    println!("All routes configured with SeaORM");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\nServer running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
