# **Tioskop – Sistem Pemesanan Tiket Bioskop 🎬**

**A Functional Programming Approach with Rust**

**Authors:**  
Kelompok 3 - Pemrograman Fungsional A  
Aditya Ridho Nugroho | Alief Rachmattul Islam | Arya Zaky Pradipta | Muhamad Faisal | Muhammad Fatwa Al Choiri

**Repository:** https://github.com/adityaridhon/Tioskop

---

## **Abstract**

Tioskop adalah aplikasi web berbasis Rust untuk pemesanan tiket bioskop yang menerapkan prinsip-prinsip pemrograman fungsional. Aplikasi ini dibangun menggunakan **Axum** sebagai web framework backend dan **Vue.js** untuk frontend, dengan **SeaORM** sebagai Object-Relational Mapping untuk interaksi database MySQL. 

Proyek ini mengimplementasikan konsep-konsep functional programming seperti **immutability**, **pure functions**, **higher-order functions**, **pattern matching**, dan **error handling** melalui sistem tipe yang kuat (Result/Option types). Backend menggunakan **async/await** dengan Tokio runtime untuk concurrent request handling, dan menerapkan **ownership & borrowing** sebagai memory safety guarantee tanpa garbage collector.

Fitur utama meliputi: pencarian film, melihat jadwal tayang per bioskop, pemilihan kursi interaktif, dan sistem booking dengan konfirmasi pembayaran. Aplikasi ini mendemonstrasikan bagaimana paradigma functional programming dapat menghasilkan kode yang lebih aman, maintainable, dan performant dalam konteks real-world web application.

---

## **Introduction**

### **Motivation**

Di era digital saat ini, pemesanan tiket bioskop masih sering dilakukan secara manual atau melalui sistem yang kompleks dan tidak user-friendly. Kami mengembangkan **Tioskop** untuk menyederhanakan proses booking tiket dengan interface yang intuitif dan backend yang robust.

### **Why Rust?**

Kami memilih **Rust** karena:

1. **Memory Safety tanpa Garbage Collection** - Ownership system mencegah data races dan memory leaks pada compile-time
2. **High Performance** - Performa setara dengan C/C++, cocok untuk web backend yang handle concurrent requests
3. **Strong Type System** - Type safety mencegah banyak bug pada compile-time
4. **Excellent Async Support** - Tokio runtime memberikan concurrency yang efisien
5. **Modern Tooling** - Cargo, Clippy, dan Rustfmt mempermudah development

### **Why Functional Programming?**

Functional programming memberikan benefits:

- **Predictability** - Pure functions dengan immutable data mengurangi side effects
- **Testability** - Isolated functions lebih mudah di-test
- **Composability** - Higher-order functions memungkinkan code reuse yang elegant
- **Error Handling** - Result/Option types memaksa explicit error handling
- **Concurrency** - Immutability mengurangi race conditions

### **Unique Features**

- **Type-Safe Database Queries** dengan SeaORM compile-time verification
- **Zero-Cost Abstractions** - High-level code tanpa runtime overhead
- **Async/Await Pattern** - Non-blocking I/O untuk scalability
- **Pattern Matching** - Exhaustive error handling
- **Ownership System** - No dangling pointers, no null references

---

## **Background and Concepts**

### **Technology Stack**

#### **Backend:**
- **Rust** (Edition 2021) - Systems programming language
- **Axum 0.8.7** - Ergonomic web framework built on Tokio
- **SeaORM** - Async ORM dengan compile-time query verification
- **Tokio 1.48.0** - Async runtime untuk concurrent I/O
- **Tower-HTTP 0.6.6** - Middleware (CORS, logging)
- **Serde 1.0** - Serialization/deserialization framework
- **Chrono 0.4** - Date and time handling
- **Rust Decimal 1.39** - Precise decimal arithmetic untuk currency

#### **Database:**
- **MySQL 8.0** - Relational database
- **Connection Pool** - Managed by SeaORM untuk efficient resource usage

#### **Frontend:**
- **Vue.js 3** - Progressive JavaScript framework
- **Vite** - Build tool dan dev server
- **Axios** - HTTP client untuk API calls
- **Vue Router** - Client-side routing

### **Key Functional Programming Concepts**

#### **1. Immutability**
```rust
// Default immutable
let booking_code = format!("BK{}", timestamp);
// booking_code = "new"; // ERROR: cannot assign twice

// Explicit mutable
let mut counter = 0;
counter += 1; // OK
```

#### **2. Pure Functions**
```rust
// Pure function: same input always produces same output, no side effects
fn calculate_total_price(seat_count: usize, price_per_seat: Decimal) -> Decimal {
    Decimal::from(seat_count) * price_per_seat
}
```

#### **3. Higher-Order Functions**
```rust
// Functions that take/return other functions
let movie_titles: Vec<String> = movies
    .into_iter()
    .filter(|m| m.genre == Some("Action".to_string()))
    .map(|m| m.title)
    .collect();
```

#### **4. Pattern Matching**
```rust
match MoviesEntity::find_by_id(id).one(&db).await {
    Ok(Some(movie)) => Ok(Json(ApiResponse::success(movie))),
    Ok(None) => Err(StatusCode::NOT_FOUND),
    Err(e) => {
        eprintln!("Database error: {}", e);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

#### **5. Error Handling with Result/Option Types**
```rust
// Result<T, E> forces error handling
async fn find_movie(id: i64) -> Result<Movie, DbErr> {
    MoviesEntity::find_by_id(id)
        .one(&db)
        .await?  // Propagate error
        .ok_or(DbErr::RecordNotFound("Movie not found".into()))
}
```

#### **6. Ownership & Borrowing**
```rust
// Ownership transfer
fn take_ownership(data: Vec<i32>) { /* data moved here */ }

// Borrowing (no ownership transfer)
fn borrow_data(data: &Vec<i32>) { /* data borrowed */ }

// Mutable borrowing
fn modify_data(data: &mut Vec<i32>) { data.push(4); }
```

#### **7. Type System & Generics**
```rust
// Generic response type
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), message: None }
    }
}
```

---

## **Source Code and Explanation**

### **Project Structure**

```
backend/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── entities/               # Database entities (SeaORM models)
│   │   ├── mod.rs
│   │   ├── bookings.rs
│   │   ├── booking_seats.rs
│   │   ├── cinemas.rs
│   │   ├── movies.rs
│   │   ├── seats.rs
│   │   ├── showtimes.rs
│   │   ├── studios.rs
│   │   └── users.rs
│   ├── handlers/               # Request handlers
│   │   ├── mod.rs
│   │   ├── auth_handler.rs
│   │   ├── booking_handler.rs
│   │   ├── movie_handler.rs
│   │   ├── seat_handler.rs
│   │   ├── showtime_handler.rs
│   │   └── studio_handler.rs
│   ├── middleware/             # Custom middleware
│   │   ├── mod.rs
│   │   └── auth.rs
│   ├── models/                 # Request/Response models
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── response.rs
│   │   └── user.rs
│   ├── routes/                 # Route definitions
│   │   ├── mod.rs
│   │   ├── auth_routes.rs
│   │   ├── booking_routes.rs
│   │   ├── movie_routes.rs
│   │   ├── seat_routes.rs
│   │   ├── showtime_routes.rs
│   │   ├── studio_routes.rs
│   │   └── workflow_routes.rs
│   └── services/               # Business logic
│       ├── mod.rs
│       ├── booking.rs
│       ├── movie.rs
│       ├── showtime.rs
│       └── workflow_service.rs
├── Cargo.toml                  # Dependencies
└── .env                        # Environment variables
```

---

### **1. Main Entry Point**

**File:** `backend/src/main.rs`

```rust
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

    // Setup SeaORM database connection (ONLY ONE CONNECTION NOW!)
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let db_connection = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database with SeaORM");

    println!("✓ Connected to database with SeaORM");

    // Create workflow service
    let workflow_service = Arc::new(JadwalWorkflowService::new(db_connection.clone()));

    let workflow_state = WorkflowAppState {
        workflow_service: workflow_service.clone(),
    };

    println!("✓ Workflow service initialized");

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Build all routes with SeaORM DatabaseConnection
    let app_routes = Router::new()
        .merge(auth_routes())
        .merge(movie_routes())
        .merge(showtime_routes())
        .merge(studio_routes())
        .merge(seat_routes())
        .merge(booking_routes())
        .with_state(db_connection);

    // Build workflow routes with SeaORM
    let workflow_router = Router::new()
        .nest("/api/workflow", workflow_routes::workflow_routes())
        .with_state(workflow_state);

    // Combine all routes
    let app = Router::new()
        .merge(app_routes)
        .merge(workflow_router)
        .layer(cors);

    println!("✓ All routes configured with SeaORM");

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("\n🚀 Server running on http://{}", addr);
    println!("\n📋 API Endpoints:");
    println!("   Auth:");
    println!("     POST /api/auth/register");
    println!("     POST /api/auth/login");
    println!("   Movies:");
    println!("     GET    /api/movies");
    println!("     POST   /api/movies");
    println!("     GET    /api/movies/search");
    println!("     PATCH  /api/movies/:id");
    println!("     DELETE /api/movies/:id");
    println!("   Showtimes:");
    println!("     GET    /api/showtimes");
    println!("     POST   /api/showtimes");
    println!("     GET    /api/showtimes/movie/:movie_id");
    println!("     PATCH  /api/showtimes/:id");
    println!("     DELETE /api/showtimes/:id");
    println!("   Workflow:");
    println!("     GET  /api/workflow/jadwal/terdekat");
    println!("     GET  /api/workflow/jadwal/studio/:studio_id");
    println!("     GET  /api/workflow/jadwal/movie/:movie_id");
    println!("     GET  /api/workflow/jadwal/stats                           ");
    println!("     GET  /api/workflow/jadwal/batch?chunk_size=100            ");
    println!("     POST /api/workflow/jadwal/filter-kompleks                 ");
    println!("     GET  /api/workflow/jadwal/film/:movie_id/semua-bioskop    ");
    println!("     GET  /api/workflow/jadwal/semua-film                      ");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

**Explanation:**

- **Lines 1-6**: Module declarations mengorganisir code berdasarkan responsibility
- **Lines 8-24**: Import statements untuk dependencies
- **Line 26**: `#[tokio::main]` - Macro yang men-transform async fn main menjadi sync entry point dengan Tokio runtime
- **Lines 28-31**: Load environment variables dari `.env` file
- **Lines 33-35**: Establish database connection pool dengan SeaORM
- **Line 39**: `Arc<T>` - Atomic Reference Counting untuk thread-safe shared ownership
- **Lines 42-45**: Configure CORS middleware untuk allow cross-origin requests
- **Lines 47-50**: Create workflow state dengan database connection dan service
- **Lines 52-60**: Compose router dengan nested routes dan CORS layer
- **Lines 64-68**: Bind server ke address dan start listening

**Functional Programming Principles:**
- **Immutability**: Semua bindings immutable by default (lines 30, 33, 39, 42, 47, 52, 64)
- **Composition**: Router composition dengan `.nest()` method (lines 53-59)
- **Error Handling**: `.expect()` untuk explicit error messages (lines 31, 35)

---

### **2. Database Entities**

#### **Movies Entity**

**File:** `backend/src/entities/movies.rs`

```rust
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "movies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<NaiveDate>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::showtimes::Entity")]
    Showtimes,
}

impl Related<super::showtimes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Showtimes.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

```

**Explanation:**

- **Lines 1-2**: Import SeaORM prelude dan serde untuk serialization
- **Lines 4-5**: Derive macros untuk automatic trait implementation
  - `Clone`: Enable cloning
  - `Debug`: Enable debug formatting
  - `PartialEq`: Enable equality comparison
  - `DeriveEntityModel`: Generate entity, column, and primary key types
  - `Serialize/Deserialize`: JSON conversion
- **Line 5**: Attribute macro specifying database table name
- **Lines 6-17**: Model struct representing database schema
  - **Line 8**: Primary key dengan type `i64`
  - **Line 9**: Required field (no Option wrapper)
  - **Lines 10-16**: Optional fields dengan `Option<T>` untuk nullable columns
- **Lines 19-23**: Relation enum defining relationships dengan entities lain
- **Line 21**: `has_many` relationship - one movie has many showtimes
- **Lines 25-29**: Implementation untuk accessing related showtimes
- **Line 31**: Empty implementation untuk ActiveModelBehavior (hooks untuk lifecycle events)

**Functional Programming Principles:**
- **Type Safety**: Explicit `Option<T>` untuk nullable fields (no null references)
- **Immutability**: All fields are immutable by default
- **Pure Functions**: Relation methods are pure (no side effects)
- **Derive Macros**: Code generation untuk reduce boilerplate (DRY principle)

---

#### **Bookings Entity**

**File:** `backend/src/entities/bookings.rs`

```rust
use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "bookings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: Option<i64>,
    pub showtime_id: Option<i64>,
    pub booking_code: String,
    pub total_price: Option<Decimal>,
    pub payment_status: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    Users,
    #[sea_orm(
        belongs_to = "super::showtimes::Entity",
        from = "Column::ShowtimeId",
        to = "super::showtimes::Column::Id"
    )]
    Showtimes,
    #[sea_orm(has_many = "super::booking_seats::Entity")]
    BookingSeats,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::showtimes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Showtimes.def()
    }
}

impl Related<super::booking_seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingSeats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Explanation:**

- **Line 1**: Import `Decimal` type untuk precise decimal arithmetic (avoid floating point errors)
- **Line 13**: `Decimal` type untuk currency values (more accurate than `f64` for money)
- **Lines 20-25**: `belongs_to` relationship - booking belongs to one user
  - **Line 22**: Foreign key column di table ini
  - **Line 23**: Referenced column di table users
- **Lines 26-31**: `belongs_to` relationship - booking belongs to one showtime
- **Line 32**: `has_many` relationship - one booking has many booking_seats
- **Lines 36-52**: Implementation untuk accessing related entities via type-safe methods

**Functional Programming Principles:**
- **Precise Arithmetic**: `Decimal` type prevents floating point errors
- **Type-Safe Relations**: Compile-time verified foreign key relationships
- **Immutable Data**: All fields are immutable by default

---

#### **Seats Entity**

**File:** `backend/src/entities/seats.rs`

```rust
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "seats")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub studio_id: Option<i64>,
    pub seat_code: String,
    pub seat_row: Option<i32>,
    pub seat_col: Option<i32>,
    pub seat_status: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::studios::Entity",
        from = "Column::StudioId",
        to = "super::studios::Column::Id"
    )]
    Studios,
    #[sea_orm(has_many = "super::booking_seats::Entity")]
    BookingSeats,
}

impl Related<super::studios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studios.def()
    }
}

impl Related<super::booking_seats::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BookingSeats.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Explanation:**

- **Lines 10-12**: Seat identification fields
  - `seat_code`: Human-readable code (e.g., "A1", "B5")
  - `seat_row`: Row number
  - `seat_col`: Column number
- **Line 13**: Seat status (e.g., "AVAILABLE", "BROKEN")
- **Lines 18-23**: `belongs_to` relationship - seat belongs to one studio
- **Line 24**: `has_many` relationship - one seat can be in many booking_seats (historical bookings)

---

#### **Showtimes Entity**

**File:** `backend/src/entities/showtimes.rs`

```rust
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "showtimes")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub movie_id: Option<i64>,
    pub studio_id: Option<i64>,
    pub start_time: Option<DateTime<Local>>,
    pub price: Option<Decimal>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::movies::Entity",
        from = "Column::MovieId",
        to = "super::movies::Column::Id"
    )]
    Movies,
    #[sea_orm(
        belongs_to = "super::studios::Entity",
        from = "Column::StudioId",
        to = "super::studios::Column::Id"
    )]
    Studios,
    #[sea_orm(has_many = "super::bookings::Entity")]
    Bookings,
}

impl Related<super::movies::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Movies.def()
    }
}

impl Related<super::studios::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Studios.def()
    }
}

impl Related<super::bookings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Bookings.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
```

**Explanation:**

- **Line 12**: `chrono::DateTime<Utc>` for timezone-aware timestamp
- **Line 13**: `Decimal` type untuk price (currency)
- **Lines 18-23**: Showtime belongs to one movie
- **Lines 24-29**: Showtime belongs to one studio
- **Line 30**: One showtime has many bookings

---

### **3. API Handlers**

#### **Movie Handler**

**File:** `backend/src/handlers/movie_handler.rs`

```rust
use crate::models::{movie::*, response::ApiResponse, DeleteResponse};
use crate::services::movie;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use sea_orm::DatabaseConnection;

fn to_response<T>(result: Result<T, movie::MovieError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

/// Get all movies
pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_all(&db).await)
}

/// Search movies
pub async fn search(
    State(db): State<DatabaseConnection>,
    Query(params): Query<SearchParams>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::search(&db, params.q).await)
}

/// Get movie by ID
pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::get_by_id(&db, id).await)
}

/// Create movie
pub async fn create(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateMovieRequest>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::create(&db, payload).await)
}

/// Update movie
pub async fn update(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdateMovieRequest>,
) -> Json<ApiResponse<crate::entities::Movie>> {
    to_response(movie::update(&db, id, payload).await)
}

/// Delete movie
pub async fn delete_movie(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<DeleteResponse>> {
    movie::delete(&db, id)
        .await
        .map(|id| {
            Json(ApiResponse::success(
                "Berhasil menghapus film",
                DeleteResponse { id, deleted: true },
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&e.to_string())))
}
/// Get movies by genre
pub async fn get_by_genre(
    State(db): State<DatabaseConnection>,
    Path(genre): Path<String>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_by_genre(&db, &genre).await)
}

/// Get movies by rating
pub async fn get_by_rating(
    State(db): State<DatabaseConnection>,
    Path(rating): Path<String>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_by_rating(&db, &rating).await)
}

/// Get latest movies
pub async fn get_latest(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Movie>>> {
    to_response(movie::get_latest(&db, 10).await)
}
```

**Explanation:**

**Lines 17-26: Request DTO (Data Transfer Object)**
- Struct untuk deserialize JSON request body
- `Option<T>` untuk optional fields

**Lines 28-39: Get All Movies Handler**
- **Line 29**: `State(db)` - Extract database connection dari application state
- **Line 30**: Return type `Result<Json<...>, StatusCode>` untuk error handling
- **Line 31**: Async database query dengan `.await`
- **Lines 31-38**: Pattern matching untuk exhaustive error handling
  - `Ok(movies)` - Success case, wrap dalam ApiResponse
  - `Ok(None)` - No results found
  - `Err(e)` - Database error

**Lines 41-53: Get Movie by ID**
- **Line 43**: `Path(id)` - Extract ID dari URL path parameter
- **Line 45**: `find_by_id()` - Type-safe query dengan primary key
- **Line 46**: `Ok(Some(movie))` - Nested Option dalam Result

**Lines 55-73: Search Movies**
- **Line 57**: `Query(params)` - Extract query parameters dari URL
- **Line 59**: Higher-order function `.map()` dengan closure
- **Line 59**: `.unwrap_or("")` - Default value jika None
- **Line 62**: `.filter()` - Type-safe WHERE clause
- **Line 62**: `.contains()` - SQL LIKE operation

**Lines 75-101: Create Movie**
- **Line 77**: `Json(payload)` - Deserialize JSON body ke struct
- **Lines 79-81**: `.and_then()` - Monad chaining untuk Option<T>
  - Transform Option<String> ke Option<NaiveDate>
  - `.ok()` convert Result ke Option (discard error)
- **Lines 83-92**: Create ActiveModel dengan `Set()` wrapper
- **Line 91**: `..Default::default()` - Fill remaining fields dengan default
- **Line 94**: `.insert()` - Execute INSERT query

**Lines 103-137: Update Movie**
- **Lines 108-115**: Fetch existing movie first
- **Line 110**: Early return dengan `return Err(...)`
- **Line 121**: Convert Model ke ActiveModel (untuk update)
- **Line 121**: `mut` keyword karena akan modify fields
- **Lines 122-128**: Update individual fields dengan `Set()`
- **Line 130**: `.update()` - Execute UPDATE query

**Lines 139-151: Delete Movie**
- **Line 143**: `.delete_by_id()` - Type-safe DELETE operation
- **Line 144**: Guard condition `if result.rows_affected > 0`
- **Line 144**: Success returns `StatusCode::NO_CONTENT` (204)
- **Line 145**: No rows deleted returns `NOT_FOUND` (404)

**Functional Programming Principles:**
- **Immutability**: Semua bindings immutable except line 121 (`mut active_movie`)
- **Pattern Matching**: Exhaustive error handling (lines 31-38, 45-52, 61-72, etc.)
- **Higher-Order Functions**: `.map()`, `.filter()`, `.and_then()` (lines 59, 62, 79)
- **Result Type**: Explicit error handling, no exceptions
- **Option Type**: Safe handling of nullable values (lines 46, 59, 79-81)
- **Type Safety**: Compile-time verified queries

---

#### **Booking Handler**

**File:** `backend/src/handlers/booking_handler.rs`

```rust
use crate::middleware::auth::AuthUser;
use crate::models::{booking::*, response::ApiResponse};
use crate::services::booking;
use axum::{extract::{Path, State}, Json};
use sea_orm::DatabaseConnection;

fn to_response<T>(result: Result<T, booking::BookingError>) -> Json<ApiResponse<T>> {
    match result {
        Ok(data) => Json(ApiResponse::success("Success", data)),
        Err(e) => Json(ApiResponse::error(&e.to_string())),
    }
}

pub async fn get_all(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<Vec<crate::entities::Booking>>> {
    to_response(booking::get_all(&db).await)
}

pub async fn get_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<BookingDetail>> {
    to_response(booking::get_detail(&db, id).await)
}

pub async fn get_by_user(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
) -> Json<ApiResponse<Vec<crate::entities::Booking>>> {
    to_response(booking::get_by_user(&db, user_id).await)
}

pub async fn create(
    AuthUser(user_id): AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateBookingRequest>,
) -> Json<ApiResponse<BookingDetail>> {
    to_response(booking::create(&db, user_id, payload).await)
}

pub async fn update_payment(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePaymentStatusRequest>,
) -> Json<ApiResponse<crate::entities::Booking>> {
    to_response(booking::update_payment(&db, id, payload.payment_status).await)
}

pub async fn cancel(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<crate::entities::Booking>> {
    to_response(booking::cancel(&db, id).await)
}

pub async fn get_booked_seats(
    State(db): State<DatabaseConnection>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<String>>> {
    to_response(booking::get_booked_seats(&db, showtime_id).await)
}
```

**Explanation:**

**Lines 19-24: Request DTO**
- `seat_ids: Vec<i64>` - Array of seat IDs to book

**Lines 26-32: Response DTO**
- Custom response structure dengan computed fields

**Lines 34-114: Create Booking Handler (Complex Transaction)**

**Line 38-40**: Input validation
- Early return jika seat_ids empty

**Lines 42-45**: Begin database transaction
- `.begin()` - Start transaction
- `.map_err()` - Transform error type dari `DbErr` ke `StatusCode`
- `?` operator - Propagate error if failed

**Lines 47-54**: Fetch showtime
- Query executed within transaction (`&txn`)
- Chained error handling:
  - `.await` - Wait for async operation
  - `.map_err()` - Handle database error
  - `?` - Propagate error
  - `.ok_or()` - Convert `Option<T>` to `Result<T, E>`

**Lines 56-63**: Fetch seats
- `.is_in()` - SQL IN clause (WHERE id IN (...))
- `.clone()` - Clone seat_ids karena ownership

**Lines 65-68**: Validate all seats exist
- If count mismatch, rollback transaction
- Atomicity guarantee - all or nothing

**Line 70**: Generate booking code
- `format!()` macro untuk string interpolation
- `Utc::now().timestamp()` - Current Unix timestamp

**Line 71**: Calculate total price
- `Decimal::from()` - Safe conversion
- Precise multiplication (no floating point error)

**Lines 73-80**: Create booking ActiveModel
- `Set()` wrapper marks field for insertion
- `.clone()` karena booking_code akan digunakan lagi
- `..Default::default()` - Auto-generate other fields (id, timestamps)

**Lines 82-85**: Insert booking
- Execute INSERT within transaction
- `.map_err()` convert error
- `?` propagate error

**Lines 87-99**: Insert booking seats (loop)
- **Line 87**: `&seats` - Immutable borrow (tidak consume seats)
- **Lines 88-93**: Create ActiveModel untuk each seat
- **Lines 95-98**: Insert each booking_seat
- If any insert fails, entire transaction rolls back

**Lines 101-104**: Commit transaction
- Only commits if all operations succeeded
- Atomicity guarantee

**Lines 106-111**: Build response
- **Line 110**: Higher-order function chain:
  - `.iter()` - Create iterator
  - `.map(|s| s.seat_code.clone())` - Transform each seat to code
  - `.collect()` - Collect into Vec<String>

**Lines 116-126: Get All Bookings**
- Simple read operation (no transaction needed)

**Functional Programming Principles:**
- **Transaction Atomicity**: All operations succeed or none (lines 42-104)
- **Error Propagation**: `?` operator untuk clean error handling
- **Higher-Order Functions**: `.map()`, `.iter()`, `.collect()` (line 110)
- **Immutability**: Most bindings immutable
- **Type Safety**: `Decimal` untuk currency arithmetic (line 71)
- **Pattern Matching**: Exhaustive error cases (lines 119-125)

---

### **4. Business Logic Services**

#### **Workflow Service**

**File:** `backend/src/services/workflow_service.rs`

```rust

use crate::entities::{Showtime, ShowtimesEntity};
use chrono::{DateTime, Local};
use rayon::prelude::*;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use sea_orm::{DatabaseConnection, EntityTrait};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum StatusJadwal {
    Mendesak {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
        selisih_menit: i64,
    },
    Aman {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
    },
    Selesai {
        jadwal_id: i64,
        waktu_selesai: DateTime<Local>,
    },
}

#[derive(Debug)]
pub enum HasilAnalisa {
    JadwalTerdekat(StatusJadwal),
    TidakAdaJadwal,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct JadwalStatistik {
    pub total_jadwal: usize,
    pub jadwal_hari_ini: usize,
    pub jadwal_minggu_ini: usize,
    pub jadwal_mendesak: usize,
    pub harga_rata_rata: f64,
    pub harga_tertinggi: Option<Decimal>,
    pub harga_terendah: Option<Decimal>,
    pub studio_terpopuler: Option<i64>,
    pub movie_terpopuler: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct FilterKriteria {
    pub studio_id: Option<i64>,
    pub movie_id: Option<i64>,
    pub min_harga: Option<Decimal>,
    pub max_harga: Option<Decimal>,
    pub hari_ini_saja: bool,
    pub hanya_mendesak: bool,
}

pub struct JadwalWorkflowService {
    db: DatabaseConnection,
}

impl JadwalWorkflowService {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    // ========================================================================
    // DATABASE LAYER - SeaORM
    // ========================================================================
    
    pub async fn fetch_jadwal_dari_db(&self) -> Result<Vec<Showtime>, sea_orm::DbErr> {
        ShowtimesEntity::find().all(&self.db).await
    }

    // ========================================================================
    // RAYON MULTIPROCESSING - PARALLEL OPERATIONS
    // ========================================================================
    
    /// RAYON: Cari jadwal terdekat dengan parallel processing
    pub fn cari_jadwal_terdekat(jadwal_slice: &[Showtime]) -> HasilAnalisa {
        if jadwal_slice.is_empty() {
            return HasilAnalisa::TidakAdaJadwal;
        }

        let waktu_sekarang = Local::now();

        let jadwal_terdekat = jadwal_slice
            .par_iter()
            .filter_map(|jadwal| jadwal.start_time.map(|st| (jadwal, st)))
            .filter(|(_, start_time)| *start_time > waktu_sekarang)
            .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());

        match jadwal_terdekat {
            Some((jadwal, start_time)) => {
                let selisih = start_time - waktu_sekarang;
                let selisih_menit = selisih.num_minutes();

                if selisih_menit <= 30 {
                    HasilAnalisa::JadwalTerdekat(StatusJadwal::Mendesak {
                        jadwal_id: jadwal.id,
                        waktu_mulai: start_time,
                        selisih_menit,
                    })
                } else {
                    HasilAnalisa::JadwalTerdekat(StatusJadwal::Aman {
                        jadwal_id: jadwal.id,
                        waktu_mulai: start_time,
                    })
                }
            }
            None => HasilAnalisa::TidakAdaJadwal,
        }
    }

    /// RAYON: Filter jadwal terdekat per film di semua bioskop (parallel grouping)
    pub fn filter_jadwal_terdekat_per_film(jadwal_slice: &[Showtime]) -> Vec<(i64, StatusJadwal)> {
        let waktu_sekarang = Local::now();
        
        // RAYON: Parallel grouping by movie_id
        let jadwal_by_movie: HashMap<i64, Vec<&Showtime>> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.movie_id.map(|mid| (mid, j)))
            .fold(
                HashMap::new,
                |mut map: HashMap<i64, Vec<&Showtime>>, (movie_id, jadwal)| {
                    map.entry(movie_id).or_insert_with(Vec::new).push(jadwal);
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, mut v) in map2 {
                        map1.entry(k).or_insert_with(Vec::new).append(&mut v);
                    }
                    map1
                },
            );
        
        // RAYON: Parallel search jadwal terdekat per film
        jadwal_by_movie
            .par_iter()
            .filter_map(|(movie_id, jadwal_list)| {
                let jadwal_terdekat = jadwal_list
                    .par_iter()
                    .filter_map(|jadwal| jadwal.start_time.map(|st| (*jadwal, st)))
                    .filter(|(_, start_time)| *start_time > waktu_sekarang)
                    .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());
                
                jadwal_terdekat.map(|(jadwal, start_time)| {
                    let selisih = start_time - waktu_sekarang;
                    let selisih_menit = selisih.num_minutes();
                    
                    let status = if selisih_menit <= 30 {
                        StatusJadwal::Mendesak {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                            selisih_menit,
                        }
                    } else {
                        StatusJadwal::Aman {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                        }
                    };
                    
                    (*movie_id, status)
                })
            })
            .collect()
    }
    
    /// RAYON: Filter jadwal terdekat untuk 1 film di semua bioskop (parallel grouping)
    pub fn filter_jadwal_terdekat_film_semua_bioskop(
        jadwal_slice: &[Showtime],
        movie_id: i64,
    ) -> Vec<(i64, StatusJadwal)> {
        let waktu_sekarang = Local::now();
        
        // RAYON: Parallel filter & grouping by studio
        let jadwal_by_studio: HashMap<i64, Vec<&Showtime>> = jadwal_slice
            .par_iter()
            .filter(|j| j.movie_id == Some(movie_id))
            .filter_map(|j| j.studio_id.map(|sid| (sid, j)))
            .fold(
                HashMap::new,
                |mut map: HashMap<i64, Vec<&Showtime>>, (studio_id, jadwal)| {
                    map.entry(studio_id).or_insert_with(Vec::new).push(jadwal);
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, mut v) in map2 {
                        map1.entry(k).or_insert_with(Vec::new).append(&mut v);
                    }
                    map1
                },
            );
        
        // RAYON: Parallel search jadwal terdekat per bioskop
        jadwal_by_studio
            .par_iter()
            .filter_map(|(studio_id, jadwal_list)| {
                let jadwal_terdekat = jadwal_list
                    .par_iter()
                    .filter_map(|jadwal| jadwal.start_time.map(|st| (*jadwal, st)))
                    .filter(|(_, start_time)| *start_time > waktu_sekarang)
                    .min_by_key(|(_, start_time)| (*start_time - waktu_sekarang).num_seconds());
                
                jadwal_terdekat.map(|(jadwal, start_time)| {
                    let selisih = start_time - waktu_sekarang;
                    let selisih_menit = selisih.num_minutes();
                    
                    let status = if selisih_menit <= 30 {
                        StatusJadwal::Mendesak {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                            selisih_menit,
                        }
                    } else {
                        StatusJadwal::Aman {
                            jadwal_id: jadwal.id,
                            waktu_mulai: start_time,
                        }
                    };
                    
                    (*studio_id, status)
                })
            })
            .collect()
    }

    /// RAYON: Multi-level filtering dengan parallel execution
    pub fn filter_kompleks(jadwal_slice: &[Showtime], kriteria: &FilterKriteria) -> Vec<Showtime> {
        let waktu_sekarang = Local::now();
        
        jadwal_slice
            .par_iter()
            .filter(|jadwal| {
                if let Some(studio_id) = kriteria.studio_id {
                    if jadwal.studio_id != Some(studio_id) {
                        return false;
                    }
                }

                if let Some(movie_id) = kriteria.movie_id {
                    if jadwal.movie_id != Some(movie_id) {
                        return false;
                    }
                }

                if let Some(min_harga) = kriteria.min_harga {
                    if let Some(price) = jadwal.price {
                        if price < min_harga {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if let Some(max_harga) = kriteria.max_harga {
                    if let Some(price) = jadwal.price {
                        if price > max_harga {
                            return false;
                        }
                    }
                }

                if kriteria.hari_ini_saja {
                    if let Some(start_time) = jadwal.start_time {
                        if start_time.date_naive() != waktu_sekarang.date_naive() {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                if kriteria.hanya_mendesak {
                    if let Some(start_time) = jadwal.start_time {
                        let selisih = (start_time - waktu_sekarang).num_minutes();
                        if selisih > 30 || selisih < 0 {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect()
    }

    /// RAYON: Agregasi statistik dengan parallel reduce
    pub fn hitung_statistik(jadwal_slice: &[Showtime]) -> JadwalStatistik {
        let waktu_sekarang = Local::now();
        let hari_ini = waktu_sekarang.date_naive();
        let minggu_ini_start = waktu_sekarang.date_naive() - chrono::Duration::days(7);

        let (
            total_jadwal,
            jadwal_hari_ini,
            jadwal_minggu_ini,
            jadwal_mendesak,
            total_harga,
            count_harga,
        ) = jadwal_slice
            .par_iter()
            .fold(
                || (0usize, 0usize, 0usize, 0usize, Decimal::ZERO, 0usize),
                |(total, hari_ini_count, minggu_ini_count, mendesak, mut harga_sum, harga_count), jadwal| {
                    let new_total = total + 1;
                    let mut new_hari_ini = hari_ini_count;
                    let mut new_minggu_ini = minggu_ini_count;
                    let mut new_mendesak = mendesak;
                    let mut new_harga_count = harga_count;

                    if let Some(start_time) = jadwal.start_time {
                        if start_time.date_naive() == hari_ini {
                            new_hari_ini += 1;
                        }
                        if start_time.date_naive() >= minggu_ini_start {
                            new_minggu_ini += 1;
                        }

                        let selisih = (start_time - waktu_sekarang).num_minutes();
                        if selisih > 0 && selisih <= 30 {
                            new_mendesak += 1;
                        }
                    }

                    if let Some(price) = jadwal.price {
                        harga_sum += price;
                        new_harga_count += 1;
                    }

                    (new_total, new_hari_ini, new_minggu_ini, new_mendesak, harga_sum, new_harga_count)
                },
            )
            .reduce(
                || (0, 0, 0, 0, Decimal::ZERO, 0),
                |(t1, h1, m1, md1, hs1, hc1), (t2, h2, m2, md2, hs2, hc2)| {
                    (t1 + t2, h1 + h2, m1 + m2, md1 + md2, hs1 + hs2, hc1 + hc2)
                },
            );

        let harga_tertinggi = jadwal_slice
            .par_iter()
            .filter_map(|j| j.price)
            .max();

        let harga_terendah = jadwal_slice
            .par_iter()
            .filter_map(|j| j.price)
            .min();

        let studio_terpopuler = Self::studio_terpopuler_parallel(jadwal_slice);
        let movie_terpopuler = Self::movie_terpopuler_parallel(jadwal_slice);

        let harga_rata_rata = if count_harga > 0 {
            (total_harga / Decimal::from(count_harga)).to_f64().unwrap_or(0.0)
        } else {
            0.0
        };

        JadwalStatistik {
            total_jadwal,
            jadwal_hari_ini,
            jadwal_minggu_ini,
            jadwal_mendesak,
            harga_rata_rata,
            harga_tertinggi,
            harga_terendah,
            studio_terpopuler,
            movie_terpopuler,
        }
    }

    /// RAYON: Cari studio terpopuler dengan parallel grouping
    fn studio_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        let studio_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.studio_id)
            .fold(
                HashMap::new,
                |mut map, studio_id| {
                    *map.entry(studio_id).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, v) in map2 {
                        *map1.entry(k).or_insert(0) += v;
                    }
                    map1
                },
            );

        studio_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(studio_id, _)| studio_id)
    }

    /// RAYON: Cari movie terpopuler dengan parallel grouping
    fn movie_terpopuler_parallel(jadwal_slice: &[Showtime]) -> Option<i64> {
        let movie_counts: HashMap<i64, usize> = jadwal_slice
            .par_iter()
            .filter_map(|j| j.movie_id)
            .fold(
                HashMap::new,
                |mut map, movie_id| {
                    *map.entry(movie_id).or_insert(0) += 1;
                    map
                },
            )
            .reduce(
                HashMap::new,
                |mut map1, map2| {
                    for (k, v) in map2 {
                        *map1.entry(k).or_insert(0) += v;
                    }
                    map1
                },
            );

        movie_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(movie_id, _)| movie_id)
    }

    /// RAYON: Filter by studio
    pub fn filter_by_studio(jadwal_slice: &[Showtime], studio_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.studio_id == Some(studio_id))
            .collect()
    }

    /// RAYON: Filter by movie
    pub fn filter_by_movie(jadwal_slice: &[Showtime], movie_id: i64) -> Vec<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.movie_id == Some(movie_id))
            .collect()
    }

    pub fn count_jadwal(jadwal_slice: &[Showtime]) -> usize {
        jadwal_slice.len()
    }

    /// RAYON: Cari jadwal dengan harga tertinggi
    pub fn jadwal_harga_tertinggi(jadwal_slice: &[Showtime]) -> Option<&Showtime> {
        jadwal_slice
            .par_iter()
            .filter(|jadwal| jadwal.price.is_some())
            .max_by_key(|jadwal| jadwal.price.unwrap())
    }

    // ========================================================================
    // WORKFLOW ORCHESTRATION
    // ========================================================================

    pub async fn execute_workflow(&self) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let jadwal_slice = jadwal_vec.as_slice();
        let hasil = Self::cari_jadwal_terdekat(jadwal_slice);

        Ok(hasil)
    }

    pub async fn execute_workflow_by_studio(&self, studio_id: i64) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_studio(jadwal_vec.as_slice(), studio_id);
        let owned_jadwal: Vec<Showtime> = filtered.into_iter().cloned().collect();
        let hasil = Self::cari_jadwal_terdekat(owned_jadwal.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_by_movie(&self, movie_id: i64) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_by_movie(jadwal_vec.as_slice(), movie_id);
        let owned_jadwal: Vec<Showtime> = filtered.into_iter().cloned().collect();
        let hasil = Self::cari_jadwal_terdekat(owned_jadwal.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_kompleks(&self, kriteria: FilterKriteria) -> Result<HasilAnalisa, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let filtered = Self::filter_kompleks(jadwal_vec.as_slice(), &kriteria);
        let hasil = Self::cari_jadwal_terdekat(filtered.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_jadwal_film_semua_bioskop(
        &self,
        movie_id: i64,
    ) -> Result<Vec<(i64, StatusJadwal)>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_film_semua_bioskop(jadwal_vec.as_slice(), movie_id);

        Ok(hasil)
    }

    pub async fn execute_workflow_jadwal_semua_film(&self) -> Result<Vec<(i64, StatusJadwal)>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil = Self::filter_jadwal_terdekat_per_film(jadwal_vec.as_slice());

        Ok(hasil)
    }

    pub async fn execute_workflow_batch(&self, chunk_size: usize) -> Result<Vec<HasilAnalisa>, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let hasil_batch = jadwal_vec
            .as_slice()
            .par_chunks(chunk_size)
            .map(|chunk| Self::cari_jadwal_terdekat(chunk))
            .collect();

        Ok(hasil_batch)
    }

    pub async fn execute_workflow_statistik(&self) -> Result<JadwalStatistik, String> {
        let jadwal_vec = self
            .fetch_jadwal_dari_db()
            .await
            .map_err(|e| format!("Database error: {}", e))?;

        let statistik = Self::hitung_statistik(jadwal_vec.as_slice());

        Ok(statistik)
    }
}

```

**Explanation:**

**Lines 12-22: Response DTO**
- Aggregated data dari multiple tables
- Flattened structure untuk easy frontend consumption

**Lines 24-26: Service Struct**
- Holds database connection
- Encapsulates business logic

**Lines 28-31: Constructor**
- `new()` - Associated function (like static method)
- Returns `Self` (same as returning `JadwalWorkflowService`)

**Lines 33-74: Execute Workflow (Main Business Logic)**

**Line 34**: Get current time
- `Utc::now()` - Timezone-aware timestamp

**Lines 36-40**: Query future showtimes
- **Line 37**: `.filter()` - WHERE clause (start_time >= now)
- **Line 37**: `.gte()` - Greater than or equal operator
- **Line 38**: `.order_by_asc()` - ORDER BY ascending
- **Line 39**: `.all()` - Fetch all results
- **Line 40**: `?` - Propagate error if query fails

**Line 42**: Initialize results vector
- `mut` karena akan di-push

**Lines 44-71**: Process each showtime
- **Line 44**: For loop over showtimes

**Lines 45-47**: Fetch related movie
- **Line 45**: `find_by_id()` - Type-safe query
- **Line 46**: `.one()` - Fetch single result (returns `Option<Model>`)
- **Line 47**: `?` - Propagate error

**Lines 49-51**: Fetch related studio
- Same pattern as movie query

**Line 53**: Pattern matching dengan tuple destructuring
- `if let (Some(movie), Some(studio)) = (movie, studio)`
- Only proceed if both movie and studio exist
- Pattern matching untuk safe unwrapping

**Lines 54-56**: Fetch related cinema
- Nested query

**Lines 58-69**: Build result if cinema exists
- **Line 59**: `.push()` - Add to results vector
- **Lines 59-68**: Construct JadwalTerdekat struct
- **Line 67**: `.to_string()` - Convert Decimal to String

**Line 73**: Return results
- `Ok(results)` - Wrap dalam Result type

**Functional Programming Principles:**
- **Pure Business Logic**: No side effects (hanya data transformation)
- **Composition**: Complex query dari multiple simple queries
- **Async/Await**: Non-blocking concurrent operations
- **Result Type**: Explicit error handling with `?` operator
- **Pattern Matching**: Safe unwrapping dengan `if let` (line 53, 58)
- **Immutability**: Most bindings immutable (except `results` yang perlu mut untuk push)

---

### **5. Middleware & Authentication**

#### **Auth Middleware**

**File:** `backend/src/middleware/auth.rs`

```rust
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::Deserialize;

#[derive(Deserialize)]
struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthUser(pub i64);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Get Authorization header
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

            if !auth_header.starts_with("Bearer ") {
                return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header"));
            }

            let token = &auth_header[7..];

            let secret =
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "tioskop_dev_secret".to_string());

            let token_data = decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            )
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

            let user_id = token_data
                .claims
                .sub
                .parse::<i64>()
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token sub"))?;

            Ok(AuthUser(user_id))
        }
    }
}

```

**Explanation:**

**Line 9: Middleware Function Signature**
- `mut req: Request` - Mutable request (akan di-modify)
- `next: Next` - Next middleware/handler di chain
- Returns `Result<Response, StatusCode>`

**Lines 10-13: Extract Authorization Header**
- **Line 11**: `.headers()` - Get HTTP headers
- **Line 12**: `.get(header::AUTHORIZATION)` - Get specific header
- **Line 13**: `.and_then()` - Monad chaining
  - Transform `Option<&HeaderValue>` ke `Option<&str>`
  - `.ok()` discard error dari `.to_str()`

**Lines 15-20: Extract Bearer Token**
- **Line 15**: Pattern matching on auth_header
- **Line 16**: Guard condition `if header.starts_with("Bearer ")`
  - Matches only if header exists AND starts with "Bearer "
- **Line 17**: Remove "Bearer " prefix
- **Line 19**: Wildcard pattern `_` - No auth or invalid format

**Line 22: Get JWT Secret**
- `.unwrap_or_else()` - Lazy default value
- Closure `|_| "secret_key".to_string()` hanya executed jika Err

**Lines 24-28: Decode JWT**
- **Line 24**: `decode::<Claims>` - Generic function dengan explicit type
- **Line 25**: Token string
- **Line 26**: Decoding key dari secret bytes
- **Line 27**: Default validation rules

**Lines 29-32: Success Case**
- **Line 30**: `.extensions_mut()` - Mutable access to request extensions
- **Line 30**: `.insert()` - Store user_id for downstream handlers
- **Line 31**: `next.run(req).await` - Continue to next handler

**Line 33: Error Case**
- Return `UNAUTHORIZED` status if decode fails

**Functional Programming Principles:**
- **Higher-Order Functions**: `.and_then()`, `.unwrap_or_else()` (lines 13, 22)
- **Pattern Matching**: Token extraction (lines 15-20), JWT decode (lines 24-34)
- **Error Handling**: Result type dengan explicit cases
- **Closure**: Lazy evaluation dalam `.unwrap_or_else()` (line 22)

---

#### **Auth Handler**

**File:** `backend/src/handlers/auth_handler.rs`

```rust
use crate::middleware::auth::AuthUser;
use crate::models::*;
use crate::entities::UsersEntity;
use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, PaginatorTrait};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Simple password hashing
fn hash_password(password: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

// Simple token generation
fn generate_token(user_id: i64) -> Result<String, String> {
    // JWT with simple HMAC secret from env (fallback for dev)
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "tioskop_dev_secret".to_string());

    // token expiry: 24 hours from now
    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("JWT encode error: {}", e))
}

// Register new user fn
pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> Json<ApiResponse<UserInfo>> {
    use crate::entities::users::Column;

    if !payload.email.contains('@') {
        return Json(ApiResponse::error("Email tidak valid"));
    }

    if payload.password.len() < 6 {
        return Json(ApiResponse::error("Password minimal 6 karakter"));
    }

    let hashed_password = hash_password(&payload.password);
    let role = payload
        .role
        .unwrap_or_else(|| "customer".to_string())
        .to_uppercase();

    if role != "ADMIN" && role != "CUSTOMER" {
        return Json(ApiResponse::error("Role harus 'admin' atau 'customer'"));
    }

    // Check if email exists
    let exists = UsersEntity::find()
        .filter(Column::Email.eq(&payload.email))
        .count(&db)
        .await
        .unwrap_or(0);

    if exists > 0 {
        return Json(ApiResponse::error("Email sudah terdaftar"));
    }

    // Create new user
    use crate::entities::users::ActiveModel;
    let new_user = ActiveModel {
        name: Set(payload.name.clone()),
        email: Set(payload.email.clone()),
        password: Set(hashed_password),
        role: Set(role.clone()),
        ..Default::default()
    };

    match new_user.insert(&db).await {
        Ok(user) => {
            let user_info = UserInfo {
                id: user.id,
                name: payload.name,
                email: payload.email,
                role,
                cinema_id: None,
            };

            Json(ApiResponse::success("Registrasi berhasil", user_info))
        }
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Login user
pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    use crate::entities::users::Column;
    use sea_orm::QuerySelect;

    let hashed_password = hash_password(&payload.password);

    // Select only needed columns, skip timestamp columns
    let user_result = UsersEntity::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Name)
        .column(Column::Email)
        .column(Column::Role)
        .filter(Column::Email.eq(&payload.email))
        .filter(Column::Password.eq(&hashed_password))
        .into_tuple::<(i64, String, String, String)>()
        .one(&db)
        .await;

    match user_result {
        Ok(Some((id, name, email, role))) => {
            // TODO: Implement cinema lookup when cinemas entity is created
            let cinema_id = None;

            let user_info = UserInfo {
                id,
                name,
                email,
                role,
                cinema_id,
            };

            let token = match generate_token(id) {
                Ok(t) => t,
                Err(e) => {
                    return Json(ApiResponse::error(&format!(
                        "Token generation error: {}",
                        e
                    )));
                }
            };

            let login_response = LoginResponse {
                user: user_info,
                token,
            };

            Json(ApiResponse::success("Login berhasil", login_response))
        }
        Ok(None) => Json(ApiResponse::error("Email atau password salah")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get user profile
pub async fn get_profile(
    State(db): State<DatabaseConnection>,
    AuthUser(user_id): AuthUser,
) -> Json<ApiResponse<UserInfo>> {
    use crate::entities::users::Column;
    use sea_orm::QuerySelect;

    // Select only needed columns, skip timestamp columns
    let user_result = UsersEntity::find_by_id(user_id)
        .select_only()
        .column(Column::Id)
        .column(Column::Name)
        .column(Column::Email)
        .column(Column::Role)
        .into_tuple::<(i64, String, String, String)>()
        .one(&db)
        .await;

    match user_result {
        Ok(Some((id, name, email, role))) => {
            // TODO: Implement cinema lookup when cinemas entity is created
            let cinema_id = None;

            let user_info = UserInfo {
                id,
                name,
                email,
                role,
                cinema_id,
            };

            Json(ApiResponse::success(
                "Berhasil mengambil profile",
                user_info,
            ))
        }
        Ok(None) => Json(ApiResponse::error("User tidak ditemukan")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get cinemas by admin
pub async fn get_admin_cinemas(
    State(db): State<DatabaseConnection>,
    user_id: i64,
) -> Json<ApiResponse<Vec<crate::models::studio::Cinema>>> {
    // TODO: Implement when Cinemas entity is created
    // For now, return empty array
    Json(ApiResponse::success("Berhasil mengambil cinemas", vec![]))
}

```

**Explanation:**

**Lines 15-54: Register Handler**

**Lines 19-23**: Check if email exists
- **Line 20**: `.filter()` - WHERE clause (email = payload.email)
- **Line 21**: `.one()` - Fetch single result
- **Line 23**: `.map_err()` - Convert DbErr ke StatusCode
- **Line 23**: `?` - Propagate error

**Lines 25-27**: Validate email uniqueness
- `.is_some()` - Check if Option contains value
- Return `CONFLICT` (409) if email exists

**Lines 29-30**: Hash password
- `bcrypt::hash()` - Cryptographic hash dengan salt
- `DEFAULT_COST` - Work factor (12 rounds)
- `.map_err()` - Convert bcrypt error

**Lines 32-38**: Create user ActiveModel
- **Line 33**: `.clone()` - Clone karena payload akan consumed
- **Line 36**: Default role "customer"
- **Line 37**: `..Default::default()` - Auto-fill other fields

**Lines 40-43**: Insert user
- **Line 40**: Method chaining
- **Line 41**: `.insert(&db)` - Execute INSERT
- **Line 42**: `.await` - Wait for async operation
- **Line 43**: `.map_err()` - Error handling, `?` - Propagate

**Lines 45-51**: Build user info response
- No password field (security)

**Line 53**: Return success response

**Lines 56-100: Login Handler**

**Lines 60-65**: Find user by email
- Chained operations:
  - `.one(&db).await` - Query
  - `.map_err()` - Handle query error
  - `.ok_or()` - Convert Option to Result
    - If None, return UNAUTHORIZED
  - `?` - Propagate errors

**Lines 67-68**: Verify password
- `bcrypt::verify()` - Compare plaintext dengan hash
- Constant-time comparison (prevent timing attacks)

**Lines 70-72**: Check password validity
- Early return if invalid

**Lines 74-79**: Create JWT claims
- **Line 75**: User ID
- **Line 76-77**: `.clone()` untuk move into claims
- **Line 78**: Expiration time (24 hours dari now)
  - `Utc::now()` - Current time
  - `+ chrono::Duration::hours(24)` - Add duration
  - `.timestamp()` - Convert to Unix timestamp
  - `as usize` - Type cast

**Lines 81-87**: Generate JWT token
- **Line 81**: Get secret dari env var
- **Lines 82-86**: `jsonwebtoken::encode()`
  - Header (algorithm, type)
  - Claims (payload)
  - Encoding key (secret)
- **Line 87**: `.map_err()` - Error handling, `?` - Propagate

**Lines 89-95**: Build user info

**Line 97**: Build login response (token + user)

**Line 99**: Return success

**Functional Programming Principles:**
- **Error Handling**: `.map_err()` + `?` pattern throughout
- **Chaining**: Method chaining untuk sequential operations (lines 40-43, 60-65)
- **Pure Functions**: `hash()`, `verify()` are pure
- **Type Safety**: Strong types untuk security (no string passwords in response)
- **Immutability**: Most bindings immutable

---

### **6. API Routes**

#### **Movie Routes**

**File:** `backend/src/routes/movie_routes.rs`

```rust
use crate::handlers::movie_handler::*;
use crate::handlers::update_posters::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn movie_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/movies/all", get(get_all))
        .route("/api/movies", get(search).post(create))
        .route("/api/movies/{id}", 
            get(get_by_id)
                .put(update)
                .delete(delete_movie)  
        )
        .route("/api/movies/latest", get(get_latest))
        .route("/api/movies/genre/{genre}", get(get_by_genre))
        .route("/api/movies/rating/{rating}", get(get_by_rating))
        .route("/api/movies/update-posters", post(update_movie_posters))
}
```

**Explanation:**

**Lines 1-3: Import Handlers**
- Import semua movie handler functions dari module `movie_handler`
- Functions ini adalah handler untuk setiap endpoint

**Lines 4-7: Import Axum Components**
- **Line 5**: `routing::get` - HTTP GET method builder
- **Line 6**: `Router` - Main routing component dari Axum

**Line 8: Import Database Connection**
- `DatabaseConnection` dari SeaORM untuk database operations

**Line 10: Function Signature**
- `pub fn movie_routes` - Public function untuk export ke main.rs
- Parameter: `db: DatabaseConnection` - Database connection yang akan di-share
- Return type: `Router` - Axum router dengan configured routes

**Lines 11-16: Router Configuration**
- **Line 11**: `Router::new()` - Create new router instance
- **Line 12**: Route "/" dengan multiple HTTP methods
  - `get(get_all_movies)` - GET /api/movies → Fetch all movies
  - `.post(create_movie)` - POST /api/movies → Create new movie
  - Method chaining untuk combine multiple HTTP methods pada satu path
- **Line 13**: Route "/search"
  - `get(search_movies)` - GET /api/movies/search?q=query
  - Dedicated search endpoint dengan query parameter
- **Line 14**: Route "/:id" dengan dynamic path parameter
  - `get(get_movie_by_id)` - GET /api/movies/:id → Get single movie
  - `.patch(update_movie)` - PATCH /api/movies/:id → Update movie
  - `.delete(delete_movie)` - DELETE /api/movies/:id → Delete movie
  - `:id` adalah path parameter yang akan di-extract oleh handler
- **Line 15**: `.with_state(db)` - Inject database connection ke semua routes
  - Membuat `db` accessible via `State` extractor di handlers
  - Shared state pattern untuk dependency injection

**Design Patterns:**
- **RESTful API** - Standard HTTP methods dan resource-based URLs
- **Method Chaining** - Fluent API untuk readable route configuration
- **Dependency Injection** - Database connection via `.with_state()`

---

#### **Booking Routes**

**File:** `backend/src/routes/booking_routes.rs`

```rust
use crate::handlers::booking_handler::*;
use axum::{
    Router,
    routing::{get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn booking_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/bookings", get(get_all).post(create))
        .route("/api/bookings/{id}", get(get_by_id))
        .route("/api/bookings/user/{user_id}", get(get_by_user))
        .route("/api/bookings/{id}/payment", put(update_payment))
        .route("/api/bookings/{id}/cancel", put(cancel))
        .route(
            "/api/bookings/showtime/{showtime_id}/seats",
            get(get_booked_seats),
        )
}

```

**Explanation:**

**Line 1: Import Handlers**
- `create_booking` - Handler untuk membuat booking baru
- `get_bookings` - Handler untuk fetch semua bookings

**Lines 8-12: Router Configuration**
- **Line 9**: Create new router
- **Line 10**: Single route "/" dengan dua methods
  - `get(get_bookings)` - GET /api/bookings → List all bookings
  - `.post(create_booking)` - POST /api/bookings → Create booking
- **Line 11**: Inject database connection

**Functional Programming Highlight:**
- Function composition dengan method chaining
- Immutable router building (each method returns new router)

---

#### **Showtime Routes**

**File:** `backend/src/routes/showtime_routes.rs`

```rust
use crate::handlers::showtime_handler::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn showtime_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route(
            "/api/showtimes",
            get(get_all_showtimes).post(create_showtime),
        )
        .route(
            "/api/showtimes/movie/{movie_id}",
            get(get_showtimes_by_movie),
        )
        .route(
            "/api/showtimes/{id}",
            put(update_showtime).delete(delete_showtime),
        )
}

```

**Explanation:**

**Lines 1-3: Import Handlers**
- `create_showtime` - Create new showtime
- `get_all_showtimes` - Get all showtimes
- `get_showtimes_by_movie_id` - Get showtimes filtered by movie

**Lines 10-15: Router Configuration**
- **Line 12**: Root route "/" 
  - GET untuk list all showtimes
  - POST untuk create new showtime
- **Line 13**: Route "/movie/:movie_id"
  - GET /api/showtimes/movie/1 → Get showtimes for movie ID 1
  - Path parameter `:movie_id` di-extract sebagai `Path<i64>` di handler

**RESTful Design:**
- Sub-resource routing dengan `/movie/:movie_id`
- Clear relationship antara resources (showtimes belongs to movie)

---

#### **Seat Routes**

**File:** `backend/src/routes/seat_routes.rs`

```rust
use crate::handlers::seat_handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn seat_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/seats", get(get_all_seats))
        .route("/api/seats/generate", post(generate_seats_for_studio))
        .route("/api/seats/studio/{studio_id}", get(get_seats_by_studio))
        .route(
            "/api/seats/showtime/{showtime_id}",
            get(get_seats_by_showtime),
        )
        .route(
            "/api/seats/showtime/{showtime_id}/available",
            get(get_available_seats_by_showtime),
        )
}

```

**Explanation:**

**Line 1: Import Handler**
- `get_seats_by_showtime` - Fetch available seats untuk showtime tertentu

**Lines 5-9: Router Configuration**
- **Line 7**: Single route "/showtime/:showtime_id"
  - GET /api/seats/showtime/1 → Get seats for showtime ID 1
  - Menampilkan seat availability untuk booking interface

**Use Case:**
- Frontend memanggil endpoint ini saat user memilih showtime
- Menampilkan seat map dengan status (AVAILABLE/BOOKED/BROKEN)

---

#### **Studio Routes**

**File:** `backend/src/routes/studio_routes.rs`

```rust
use crate::handlers::studio_handler::*;
use axum::{
    Router,
    routing::{delete, get, post, put},
};
use sea_orm::DatabaseConnection;

pub fn studio_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/studios", get(get_all_studios).post(create_studio))
        .route(
            "/api/studios/{id}",
            get(get_studio_by_id)
                .put(update_studio)
                .delete(delete_studio),
        )
        .route(
            "/api/studios/cinema/{cinema_id}",
            get(get_studios_by_cinema),
        )
}

```

**Explanation:**

**Line 1: Import Handler**
- `get_studios_by_cinema_id` - Fetch studios dalam satu cinema

**Lines 5-9: Router Configuration**
- **Line 7**: Route "/cinema/:cinema_id"
  - GET /api/studios/cinema/1 → Get studios for cinema ID 1
  - Hierarchical resource relationship (studios belongs to cinema)

**Data Hierarchy:**
```
Cinema (XXI BOS Mall)
  └── Studio 1
  └── Studio 2
  └── Studio 3
```

---

#### **Auth Routes**

**File:** `backend/src/routes/auth_routes.rs`

```rust
use crate::handlers::auth_handler::*;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

pub fn auth_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/profile", get(get_profile))
}

```

**Explanation:**

**Line 1: Import Auth Handlers**
- `register_handler` - User registration
- `login_handler` - User login (returns JWT token)
- `logout_handler` - User logout (invalidate token)
- `user_info_handler` - Get current user info

**Line 2: Import Auth Middleware**
- `auth_middleware` - Middleware untuk verify JWT token

**Lines 4-5: Import Axum Components**
- `middleware as axum_middleware` - Middleware utilities
- `routing::{get, post}` - HTTP method builders

**Lines 10-20: Router Configuration**
- **Line 12**: POST /api/auth/register
  - Public endpoint (no authentication required)
  - Register new user dengan name, email, password
- **Line 13**: POST /api/auth/login
  - Public endpoint
  - Login dengan email & password, returns JWT token
- **Line 14**: POST /api/auth/logout
  - Logout (optional, JWT stateless)
- **Lines 15-18**: GET /api/auth/user
  - **Protected endpoint** dengan auth middleware
  - `.layer(axum_middleware::from_fn(auth_middleware))`
  - Middleware runs before handler, verifies JWT token
  - Returns user info jika token valid

**Middleware Layer Pattern:**
```
Request → auth_middleware → user_info_handler → Response
              ↓ (if invalid token)
           401 Unauthorized
```

**Security Flow:**
1. User login dengan credentials
2. Backend verify credentials, generate JWT token
3. Frontend store token (localStorage/cookie)
4. Frontend send token di Authorization header untuk protected endpoints
5. Middleware extract & verify token
6. If valid: inject user_id ke request, proceed to handler
7. If invalid: return 401 Unauthorized

---

#### **Workflow Routes**

**File:** `backend/src/routes/workflow_routes.rs`

```rust
use chrono::{DateTime, Local};
use rust_decimal::Decimal;
use serde::Serialize;
use crate::services::workflow_service::{HasilAnalisa, JadwalWorkflowService, StatusJadwal};
use axum::{
    Json, Router, extract::Path, extract::State, http::StatusCode, response::IntoResponse,
    routing::get,
};
use std::sync::Arc;

#[derive(Clone, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum StatusJadwalResponse {
    Mendesak {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
        selisih_menit: i64,
    },
    Aman {
        jadwal_id: i64,
        waktu_mulai: DateTime<Local>,
    },
    Selesai {
        jadwal_id: i64,
        waktu_selesai: DateTime<Local>,
    },
}

#[derive(Serialize)]
#[serde(tag = "status", content = "result")]
pub enum ApiResponse {
    Success(StatusJadwalResponse),
    NoSchedule,
    Error { message: String },
}

impl From<StatusJadwal> for StatusJadwalResponse {
    fn from(status: StatusJadwal) -> Self {
        match status {
            StatusJadwal::Mendesak {
                jadwal_id,
                waktu_mulai,
                selisih_menit,
            } => StatusJadwalResponse::Mendesak {
                jadwal_id,
                waktu_mulai,
                selisih_menit,
            },
            StatusJadwal::Aman {
                jadwal_id,
                waktu_mulai,
            } => StatusJadwalResponse::Aman {
                jadwal_id,
                waktu_mulai,
            },
            StatusJadwal::Selesai {
                jadwal_id,
                waktu_selesai,
            } => StatusJadwalResponse::Selesai {
                jadwal_id,
                waktu_selesai,
            },
        }
    }
}

impl From<HasilAnalisa> for ApiResponse {
    fn from(hasil: HasilAnalisa) -> Self {
        match hasil {
            HasilAnalisa::JadwalTerdekat(status) => ApiResponse::Success(status.into()),
            HasilAnalisa::TidakAdaJadwal => ApiResponse::NoSchedule,
            HasilAnalisa::Error(msg) => ApiResponse::Error { message: msg },
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub workflow_service: Arc<JadwalWorkflowService>,
}

pub async fn get_jadwal_terdekat(State(state): State<AppState>) -> impl IntoResponse {
    match state.workflow_service.execute_workflow().await {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_by_studio(
    State(state): State<AppState>,
    Path(studio_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_by_studio(studio_id)
        .await
    {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_by_movie(
    State(state): State<AppState>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_by_movie(movie_id)
        .await
    {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_stats(State(state): State<AppState>) -> impl IntoResponse {
    match state.workflow_service.execute_workflow_statistik().await {
        Ok(statistik) => {
            let response = serde_json::json!({
                "success": true,
                "statistik": {
                    "total_jadwal": statistik.total_jadwal,
                    "jadwal_hari_ini": statistik.jadwal_hari_ini,
                    "jadwal_minggu_ini": statistik.jadwal_minggu_ini,
                    "jadwal_mendesak": statistik.jadwal_mendesak,
                    "harga_rata_rata": statistik.harga_rata_rata,
                    "harga_tertinggi": statistik.harga_tertinggi,
                    "harga_terendah": statistik.harga_terendah,
                    "studio_terpopuler": statistik.studio_terpopuler,
                    "movie_terpopuler": statistik.movie_terpopuler,
                }
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn get_jadwal_batch(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let chunk_size = params
        .get("chunk_size")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);

    match state.workflow_service.execute_workflow_batch(chunk_size).await {
        Ok(hasil_batch) => {
            let results: Vec<_> = hasil_batch
                .into_iter()
                .map(|hasil| {
                    serde_json::json!({
                        "type": match hasil {
                            HasilAnalisa::JadwalTerdekat(_) => "found",
                            HasilAnalisa::TidakAdaJadwal => "not_found",
                            HasilAnalisa::Error(_) => "error",
                        }
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "chunk_size": chunk_size,
                "total_chunks": results.len(),
                "results": results,
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn post_jadwal_filter_kompleks(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    use crate::services::workflow_service::FilterKriteria;

    let kriteria = FilterKriteria {
        studio_id: body.get("studio_id").and_then(|v| v.as_i64()),
        movie_id: body.get("movie_id").and_then(|v| v.as_i64()),
        min_harga: body.get("min_harga").and_then(|v| v.as_i64()).map(|i| Decimal::from(i)),
        max_harga: body.get("max_harga").and_then(|v| v.as_i64()).map(|i| Decimal::from(i)),
        hari_ini_saja: body.get("hari_ini_saja").and_then(|v| v.as_bool()).unwrap_or(false),
        hanya_mendesak: body.get("hanya_mendesak").and_then(|v| v.as_bool()).unwrap_or(false),
    };

    match state.workflow_service.execute_workflow_kompleks(kriteria).await {
        Ok(hasil) => {
            let response: ApiResponse = hasil.into();
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse::Error { message: e };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn get_jadwal_film_semua_bioskop(
    State(state): State<AppState>,
    Path(movie_id): Path<i64>,
) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_jadwal_film_semua_bioskop(movie_id)
        .await
    {
        Ok(hasil_list) => {
            let results: Vec<_> = hasil_list
                .into_iter()
                .map(|(studio_id, status)| {
                    let response: StatusJadwalResponse = status.into();
                    serde_json::json!({
                        "studio_id": studio_id,
                        "jadwal": response
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "movie_id": movie_id,
                "total_bioskop": results.len(),
                "jadwal_per_bioskop": results
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub async fn get_jadwal_semua_film(State(state): State<AppState>) -> impl IntoResponse {
    match state
        .workflow_service
        .execute_workflow_jadwal_semua_film()
        .await
    {
        Ok(hasil_list) => {
            let results: Vec<_> = hasil_list
                .into_iter()
                .map(|(movie_id, status)| {
                    let response: StatusJadwalResponse = status.into();
                    serde_json::json!({
                        "movie_id": movie_id,
                        "jadwal": response
                    })
                })
                .collect();

            let response = serde_json::json!({
                "success": true,
                "total_film": results.len(),
                "jadwal_per_film": results
            });

            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error = serde_json::json!({
                "success": false,
                "error": e
            });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error))
        }
    }
}

pub fn workflow_routes() -> Router<AppState> {
    Router::new()
        .route("/jadwal/terdekat", get(get_jadwal_terdekat))
        .route("/jadwal/studio/{studio_id}", get(get_jadwal_by_studio))
        .route("/jadwal/movie/{movie_id}", get(get_jadwal_by_movie))
        .route("/jadwal/stats", get(get_jadwal_stats))
        .route("/jadwal/batch", get(get_jadwal_batch))
        .route("/jadwal/filter-kompleks", axum::routing::post(post_jadwal_filter_kompleks))
        .route("/jadwal/film/{movie_id}/semua-bioskop", get(get_jadwal_film_semua_bioskop))
        .route("/jadwal/semua-film", get(get_jadwal_semua_film))
}

```

**Explanation:**

**Line 1: Import Workflow Service**
- `JadwalWorkflowService` - Business logic untuk fetch nearest showtimes

**Lines 2-7: Import Dependencies**
- **Line 3**: Axum extractors dan response types
- **Line 5**: `DatabaseConnection` from SeaORM
- **Line 6**: `serde_json::json!` macro untuk JSON construction
- **Line 7**: `Arc` untuk thread-safe shared ownership

**Lines 9-13: AppState Struct**
- **Line 9**: `#[derive(Clone)]` - Enable cloning (required for Axum state)
- **Line 10**: `pub struct AppState` - Custom state container
- **Line 11**: `pub db: DatabaseConnection` - Database connection
- **Line 12**: `pub workflow_service: Arc<JadwalWorkflowService>`
  - Wrapped dalam `Arc` untuk shared ownership across threads
  - Multiple handlers dapat akses service instance yang sama

**Lines 15-19: Router Configuration**
- **Line 15**: Function signature with custom `AppState`
- **Line 17**: Single route "/jadwal/terdekat"
  - GET /api/workflow/jadwal/terdekat
  - Handler: `get_jadwal_terdekat`
- **Line 18**: Inject custom state (bukan hanya `db`, tapi `AppState`)

**Lines 21-34: Handler Implementation**
- **Line 21**: Async handler function
- **Line 22**: Extract `AppState` dari request
  - `State(state): State<AppState>` - Pattern matching extraction
- **Line 23**: Return type `Result<impl IntoResponse, StatusCode>`
  - `impl IntoResponse` - Any type that can convert to HTTP response
  - `StatusCode` - Error type (HTTP status code)
- **Line 24**: Call workflow service
  - `state.workflow_service.execute_workflow().await`
  - Async operation, fetch nearest showtimes from database
- **Lines 25-28: Success Case**
  - Pattern match `Ok(hasil)`
  - `Json(json!({...}))` - Construct JSON response dengan macro
  - Return wrapped data dalam standard API response format
- **Lines 29-32: Error Case**
  - Pattern match `Err(e)`
  - Log error ke console dengan `eprintln!`
  - Return `StatusCode::INTERNAL_SERVER_ERROR` (500)

**Functional Programming Highlights:**

1. **Pattern Matching** - Exhaustive error handling
2. **Result Type** - No exceptions, explicit error propagation
3. **Async/Await** - Non-blocking concurrent I/O
4. **Shared State with Arc** - Thread-safe reference counting
5. **Immutable State** - `AppState` doesn't mutate, only borrowed
---

### **7. Request/Response Models**

#### **API Response Model**

**File:** `backend/src/models/response.rs`

```rust
1   use serde::Serialize;
2   
3   #[derive(Debug, Serialize)]
4   pub struct ApiResponse<T> {
5       pub success: bool,
6       pub data: Option<T>,
7       pub message: Option<String>,
8   }
9   
10  impl<T> ApiResponse<T> {
11      pub fn success(data: T) -> Self {
12          Self {
13              success: true,
14              data: Some(data),
15              message: None,
16          }
17      }
18  
19      pub fn error(message: String) -> Self {
20          Self {
21              success: false,
22              data: None,
23              message: Some(message),
24          }
25      }
26  }
```

**Explanation:**

**Line 1: Import Serde**
- `Serialize` trait untuk automatic JSON serialization

**Lines 3-8: Struct Definition**
- **Line 3**: `#[derive(Debug, Serialize)]`
  - `Debug` - Enable `{:?}` formatting untuk debugging
  - `Serialize` - Auto-implement JSON serialization
- **Line 4**: Generic struct `ApiResponse<T>`
  - `<T>` - Type parameter, bisa any data type
- **Line 5**: `success: bool` - Indicates success/failure
- **Line 6**: `data: Option<T>` - Generic data payload
  - `Option<T>` - May be `Some(T)` or `None`
- **Line 7**: `message: Option<String>` - Optional error/info message

**Lines 10-26: Implementation Block**
- **Line 10**: `impl<T>` - Implementation untuk generic type T

**Lines 11-17: Success Constructor**
- **Line 11**: Associated function `success(data: T)`
  - Takes ownership of `data`
  - Returns `Self` (ApiResponse<T>)
- **Lines 12-16**: Construct success response
  - `success: true`
  - `data: Some(data)` - Wrap data dalam Option
  - `message: None` - No message untuk success

**Lines 19-25: Error Constructor**
- **Line 19**: Associated function `error(message: String)`
  - Takes error message
  - Returns `Self`
- **Lines 20-24**: Construct error response
  - `success: false`
  - `data: None` - No data on error
  - `message: Some(message)` - Error message

**Usage Examples:**

```rust
// Success response
let movies = vec![movie1, movie2];
let response = ApiResponse::success(movies);
// JSON: { "success": true, "data": [...], "message": null }

// Error response
let response = ApiResponse::<Vec<Movie>>::error("Not found".to_string());
// JSON: { "success": false, "data": null, "message": "Not found" }
```

**Functional Programming Principles:**

1. **Generic Programming** - Works with any data type
2. **Parametric Polymorphism** - Type parameter `T`
3. **Option Type** - Explicit nullable fields (no null references)
4. **Constructor Pattern** - Named constructors untuk clarity
5. **Type Inference** - Compiler infers `T` dari usage
6. **Immutability** - Struct fields tidak bisa di-mutate setelah creation

**Type Safety Benefits:**
```rust
// Compile-time type checking
let response: ApiResponse<Vec<Movie>> = ApiResponse::success(movies);
// response.data is Option<Vec<Movie>>, not generic Option<T>

// Won't compile (type mismatch)
let response: ApiResponse<Vec<Movie>> = ApiResponse::success("string");
// ERROR: expected Vec<Movie>, found &str
```

---

#### **Auth Models**

**File:** `backend/src/models/auth.rs`

```rust
1   use serde::{Deserialize, Serialize};
2   
3   #[derive(Debug, Deserialize)]
4   pub struct RegisterRequest {
5       pub name: String,
6       pub email: String,
7       pub password: String,
8   }
9   
10  #[derive(Debug, Deserialize)]
11  pub struct LoginRequest {
12      pub email: String,
13      pub password: String,
14  }
15  
16  #[derive(Debug, Serialize)]
17  pub struct AuthResponse {
18      pub token: String,
19      pub user: UserInfo,
20  }
21  
22  #[derive(Debug, Serialize)]
23  pub struct UserInfo {
24      pub id: i64,
25      pub name: String,
26      pub email: String,
27      pub role: String,
28  }
```

**Explanation:**

**Lines 3-8: RegisterRequest**
- **Line 3**: `#[derive(Debug, Deserialize)]`
  - `Deserialize` - Parse JSON → Rust struct
- Request body untuk user registration
- All fields required (tidak ada `Option<T>`)

**Lines 10-14: LoginRequest**
- Request body untuk authentication
- Only email & password needed

**Lines 16-20: AuthResponse**
- **Line 16**: `#[derive(Debug, Serialize)]`
  - `Serialize` - Rust struct → JSON
- Response after successful login
- **Line 18**: `token: String` - JWT token
- **Line 19**: `user: UserInfo` - User details

**Lines 22-28: UserInfo**
- User information (safe to expose)
- **No password field** - Security best practice
- **Line 27**: `role: String` - User role (admin/customer)

**Security Note:**
```rust
// ❌ BAD: Exposing password hash
#[derive(Serialize)]
pub struct UserInfo {
    pub password: String, // NEVER DO THIS
}

// ✅ GOOD: Only safe fields
#[derive(Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub name: String,
    pub email: String,
}
```

---

#### **User Claims Model**

**File:** `backend/src/models/user.rs`

```rust
1   use serde::{Deserialize, Serialize};
2   
3   #[derive(Debug, Serialize, Deserialize, Clone)]
4   pub struct Claims {
5       pub user_id: i64,
6       pub email: String,
7       pub role: String,
8       pub exp: usize,
9   }
```

**Explanation:**

**Lines 3-9: JWT Claims**
- **Line 3**: `#[derive(Debug, Serialize, Deserialize, Clone)]`
  - `Serialize` - Encode to JWT
  - `Deserialize` - Decode from JWT
  - `Clone` - Allow cloning (required untuk middleware)
- **Line 5**: `user_id: i64` - Primary key
- **Line 6**: `email: String` - User identifier
- **Line 7**: `role: String` - Authorization (admin/customer)
- **Line 8**: `exp: usize` - Token expiration timestamp
  - Unix timestamp (seconds since epoch)

---

## **Screenshot**
### Dashboard Admin

Dashboard utama admin untuk mengelola sistem bioskop:

![Dashboard Admin](asset/dashboardadmin.jpeg)

**Features:**

- Overview sistem (film, jadwal, studio)
- Menu navigasi untuk manajemen data

---

### Dashboard Data Pelanggan

Interface untuk mengelola data pelanggan dan booking:

![Dashboard Data Pelanggan](asset/dashboad_datapelanggan.jpeg)

**Features:**

- List data pelanggan

---

### Dashboard Schedule

Interface untuk mengelola jadwal tayang film:

![Dashboard Schedule](asset/dashboard_schedule.jpeg)

**Features:**

- Daftar jadwal tayang (showtimes)
- Informasi film, studio, dan waktu tayang
- Harga tiket
- Management jadwal (create, update, delete)

---

## **Conclusion**
Projek ini menunjukkan bahwa Rust dapat digunakan secara efektif untuk membangun layanan booking bioskop yang memilki kebutuhan:

- Cepat & aman pada sistem concurrency yang tinggi
- Menerapkan paradigma _Functional Programming_ dengan sesuai
- Memiliki integritas data kuat melalui sistem booking atomic

---
