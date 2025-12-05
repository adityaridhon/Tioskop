mod config;
mod models;
mod handlers;
mod routes;
mod middleware;

use axum::{Router, routing::get, middleware::from_fn_with_state};
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use routes::{
    movie_routes::movie_routes, 
    showtime_routes::showtime_routes, 
    studio_routes::studio_routes, 
    seat_routes::seat_routes, 
    booking_routes::booking_routes,
    auth_routes::auth_routes
};
use handlers::cities_handler;
use middleware::city_context::city_context_middleware;

#[tokio::main]
async fn main() {
    dotenv().ok();

    println!("🚀 Starting Tioskop Multi-Database Server...\n");

    // Setup multi-database pools
    let pools = Arc::new(config::DatabasePools::new().await);
    
    println!("✅ Connected to Central DB");
    println!("✅ Loaded {} city databases", pools.cities.len());
    println!("📍 Available cities: {:?}\n", pools.list_cities());
    

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes (no city context needed - use Central DB)
    let public_routes = Router::new()
        .route("/api/cities", get(cities_handler::get_cities))
        .merge(auth_routes())
        .merge(movie_routes());

    // City-specific routes (need city context middleware)
    let city_routes = Router::new()
        .merge(showtime_routes())
        .merge(studio_routes())
        .merge(seat_routes())
        .merge(booking_routes())
        .layer(from_fn_with_state(pools.clone(), city_context_middleware));

    // Combine all routes
    let app = Router::new()
        .merge(public_routes)
        .merge(city_routes)
        .layer(cors)
        .with_state(pools);

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("🌐 Server running on http://{}", addr);
    println!("📡 API Endpoints:");
    println!("   - GET  /api/cities          (Public)");
    println!("   - POST /api/auth/login      (Public)");
    println!("   - GET  /api/movies          (Public)");
    println!("   - GET  /api/showtimes       (City-specific)");
    println!("   - POST /api/bookings        (City-specific)");
    println!("\n💡 Tip: Add 'X-City-Name' header for city-specific requests\n");
    
    
    
    
    
    
    
    
    
    
    
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
