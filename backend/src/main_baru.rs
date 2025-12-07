// ============================================================================
// MAIN.RS REVISI - Integrasi SeaORM & Workflow Service
// ============================================================================
//
// File ini berisi instruksi untuk merevisi main.rs agar support:
// - SeaORM sebagai ORM (menggantikan/melengkapi existing pool)
// - Workflow Service dengan Rayon
// - Dependency Injection pattern
//
// INSTRUKSI: Jangan hapus main.rs lama, gunakan file ini sebagai referensi
//
// ============================================================================

// STEP 1: TAMBAHKAN MODULE DECLARATIONS
// Tambahkan di bagian atas setelah mod middleware:

/*
mod config;
mod models;
mod handlers;
mod routes;
mod middleware;
mod entities;      // ← TAMBAHKAN: SeaORM entities
mod services;      // ← TAMBAHKAN: Business logic layer
*/

// ============================================================================
// STEP 2: UPDATE IMPORTS
// ============================================================================

/*
use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;  // ← TAMBAHKAN: Untuk Arc wrapper
use tower_http::cors::{Any, CorsLayer};
use sea_orm::{Database, DatabaseConnection};  // ← TAMBAHKAN: SeaORM

// Import existing routes
use routes::{
    movie_routes::movie_routes, 
    showtime_routes::showtime_routes, 
    studio_routes::studio_routes, 
    seat_routes::seat_routes, 
    booking_routes::booking_routes,
    auth_routes::auth_routes,
    workflow_routes,  // ← TAMBAHKAN: Workflow routes baru
};

// Import workflow service
use services::workflow_service::JadwalWorkflowService;
use routes::workflow_routes::AppState as WorkflowAppState;
*/

// ============================================================================
// STEP 3: SETUP DATABASE CONNECTION (SeaORM)
// ============================================================================

/*
#[tokio::main]
async fn main() {
    dotenv().ok();

    // === EXISTING DATABASE (Tetap dipertahankan untuk compatibility) ===
    let pool = config::create_pool().await;
    println!("✓ Connected to existing database pool");

    // === NEW: SeaORM DATABASE CONNECTION ===
    // Ambil database URL dari environment
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    // Buat SeaORM connection
    let db_connection: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    
    println!("✓ Connected to database with SeaORM");
    
    // === CREATE WORKFLOW SERVICE ===
    let workflow_service = Arc::new(
        JadwalWorkflowService::new(db_connection.clone())
    );
    
    // === CREATE APP STATE FOR WORKFLOW ===
    let workflow_state = WorkflowAppState {
        workflow_service: workflow_service.clone(),
    };
    
    println!("✓ Workflow service initialized");
*/

// ============================================================================
// STEP 4: SETUP CORS (Unchanged)
// ============================================================================

/*
    // Setup CORS (tetap sama)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
*/

// ============================================================================
// STEP 5: BUILD ROUTER (Add Workflow Routes)
// ============================================================================

/*
    // === BUILD ROUTER ===
    // Existing routes menggunakan pool lama (backward compatibility)
    let existing_routes = Router::new()
        .merge(auth_routes())
        .merge(movie_routes())
        .merge(showtime_routes())
        .merge(studio_routes())
        .merge(seat_routes())
        .merge(booking_routes())
        .with_state(pool);  // State untuk existing routes
    
    // NEW: Workflow routes dengan state sendiri
    let workflow_router = Router::new()
        .nest("/api/workflow", workflow_routes::workflow_routes())
        .with_state(workflow_state);
    
    // Combine all routes
    let app = Router::new()
        .merge(existing_routes)     // Existing routes
        .merge(workflow_router)      // NEW: Workflow routes
        .layer(cors);                // CORS layer di top level
    
    println!("✓ All routes configured");
    println!("  - Existing routes: /api/...");
    println!("  - Workflow routes: /api/workflow/...");
*/

// ============================================================================
// STEP 6: RUN SERVER (Unchanged)
// ============================================================================

/*
    // Run server (tetap sama)
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Server running on http://{}", addr);
    println!("\n📋 Available Workflow Endpoints:");
    println!("   GET  /api/workflow/jadwal/terdekat");
    println!("   GET  /api/workflow/jadwal/studio/:studio_id");
    println!("   GET  /api/workflow/jadwal/movie/:movie_id");
    println!("   GET  /api/workflow/jadwal/stats");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// ENVIRONMENT VARIABLES (.env)
// ============================================================================
//
// Pastikan .env file memiliki:
//
// DATABASE_URL=postgresql://user:password@localhost:5432/tioskop_db
// JWT_SECRET=tioskop_dev_secret
//
// Format DATABASE_URL untuk SeaORM:
// - PostgreSQL: postgresql://user:password@host:port/database
// - MySQL: mysql://user:password@host:port/database
// - SQLite: sqlite://path/to/database.db
//
// ============================================================================

// ============================================================================
// MIGRATION PATH (Backward Compatibility Strategy)
// ============================================================================
//
// 1. FASE 1 - Coexistence (SEKARANG)
//    - Existing routes tetap menggunakan pool lama
//    - Workflow routes baru menggunakan SeaORM
//    - Kedua sistem berjalan berdampingan
//
// 2. FASE 2 - Gradual Migration (NANTI)
//    - Migrate existing handlers satu per satu ke SeaORM
//    - Test setiap migration
//    - Monitor performance
//
// 3. FASE 3 - Full SeaORM (MASA DEPAN)
//    - Semua routes menggunakan SeaORM
//    - Remove old pool
//    - Unified state management
//
// ============================================================================

// ============================================================================
// TESTING COMMANDS
// ============================================================================
//
// 1. Build project:
//    cargo build
//
// 2. Run server:
//    cargo run
//
// 3. Test workflow endpoint:
//    curl http://localhost:3000/api/workflow/jadwal/terdekat
//
// 4. Test with specific studio:
//    curl http://localhost:3000/api/workflow/jadwal/studio/1
//
// 5. Test stats endpoint:
//    curl http://localhost:3000/api/workflow/jadwal/stats
//
// ============================================================================

// ============================================================================
// COMPLETE CODE TEMPLATE
// ============================================================================
// Uncomment seluruh blok di bawah untuk implementasi lengkap:

/*
mod config;
mod models;
mod handlers;
mod routes;
mod middleware;
mod entities;
mod services;

use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use sea_orm::{Database, DatabaseConnection};

use routes::{
    movie_routes::movie_routes, 
    showtime_routes::showtime_routes, 
    studio_routes::studio_routes, 
    seat_routes::seat_routes, 
    booking_routes::booking_routes,
    auth_routes::auth_routes,
    workflow_routes,
};

use services::workflow_service::JadwalWorkflowService;
use routes::workflow_routes::AppState as WorkflowAppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let pool = config::create_pool().await;
    println!("✓ Connected to existing database pool");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let db_connection: DatabaseConnection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");
    
    println!("✓ Connected to database with SeaORM");
    
    let workflow_service = Arc::new(
        JadwalWorkflowService::new(db_connection.clone())
    );
    
    let workflow_state = WorkflowAppState {
        workflow_service: workflow_service.clone(),
    };
    
    println!("✓ Workflow service initialized");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let existing_routes = Router::new()
        .merge(auth_routes())
        .merge(movie_routes())
        .merge(showtime_routes())
        .merge(studio_routes())
        .merge(seat_routes())
        .merge(booking_routes())
        .with_state(pool);
    
    let workflow_router = Router::new()
        .nest("/api/workflow", workflow_routes::workflow_routes())
        .with_state(workflow_state);
    
    let app = Router::new()
        .merge(existing_routes)
        .merge(workflow_router)
        .layer(cors);
    
    println!("✓ All routes configured");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Server running on http://{}", addr);
    println!("\n📋 Available Workflow Endpoints:");
    println!("   GET  /api/workflow/jadwal/terdekat");
    println!("   GET  /api/workflow/jadwal/studio/:studio_id");
    println!("   GET  /api/workflow/jadwal/movie/:movie_id");
    println!("   GET  /api/workflow/jadwal/stats");
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// CHECKLIST IMPLEMENTASI:
// ============================================================================
// [ ] 1. Tambahkan mod entities dan mod services di main.rs
// [ ] 2. Install dependencies di Cargo.toml (lihat cargo_dependencies.md)
// [ ] 3. Setup DATABASE_URL di .env
// [ ] 4. Generate entities dengan sea-orm-cli
// [ ] 5. Implementasi workflow_service.rs
// [ ] 6. Implementasi workflow_routes.rs
// [ ] 7. Update main.rs dengan SeaORM connection
// [ ] 8. Setup workflow state dan routes
// [ ] 9. Test semua endpoints
// [ ] 10. Dokumentasi API di APIDoc.md
// ============================================================================
