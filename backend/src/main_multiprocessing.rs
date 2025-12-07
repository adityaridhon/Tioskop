use axum::{
    extract::{Path, State},
    http::Method,
    routing::get,
    Json, Router,
};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, FromRow, MySqlPool};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

// ==========================================
// 1. STRUKTUR DATA (MODEL)
// ==========================================

// Model untuk mengambil konfigurasi kota dari DB Central
#[derive(FromRow, Clone)]
struct CityConfig {
    name: String,
    db_url: String, // Connection string ke DB Kota (mysql://...)
}

// Model hasil jadwal per studio
#[derive(Serialize, FromRow, Debug)]
struct ShowtimeDetail {
    cinema_name: String,
    studio_name: String,
    start_time: String, // Kita ambil sebagai String biar gampang diformat
    price: f64,
}

// Model respon akhir yang dikirim ke Vue
#[derive(Serialize, Debug)]
struct CityScheduleResponse {
    city: String,
    schedules: Vec<ShowtimeDetail>,
    status: String, // "Online" atau "Error"
}

// State aplikasi untuk menyimpan koneksi ke Central DB
struct AppState {
    central_db: MySqlPool,
}

// ==========================================
// 2. MAIN FUNCTION
// ==========================================

#[tokio::main]
async fn main() {
    // Load .env file
    dotenvy::dotenv().ok();

    // Ambil URL database central dari .env
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Buat koneksi pool ke Central DB
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Gagal connect ke Central DB");

    println!("✅ Berhasil connect ke Central DB!");

    // Bungkus state agar bisa di-share ke semua route
    let state = Arc::new(AppState { central_db: pool });

    // Setup CORS agar Vue (localhost:5173) bisa akses Axum (localhost:3000)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET]);

    // Setup Router
    let app = Router::new()
        .route("/movies/:id/schedules", get(get_movie_schedules))
        .layer(cors)
        .with_state(state);

    // Jalankan Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server berjalan di http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// ==========================================
// 3. HANDLER (LOGIKA MULTIPROCESSING)
// ==========================================

async fn get_movie_schedules(
    Path(movie_id): Path<i64>,
    State(state): State<Arc<AppState>>,
) -> Json<Vec<CityScheduleResponse>> {
    
    // A. Ambil daftar kota aktif dari Central DB
    // Query ini cepat karena cuma ambil config
    let cities = sqlx::query_as::<_, CityConfig>(
        "SELECT name, db_url FROM cities WHERE is_active = true"
    )
    .fetch_all(&state.central_db)
    .await
    .unwrap_or_else(|_| vec![]);

    println!("📡 Mencari jadwal untuk Film ID: {} di {} kota...", movie_id, cities.len());

    // B. SCATTER: Buat Task Multiprocessing (Parallel)
    // Kita map setiap kota menjadi sebuah "Future" (janji proses async)
    let tasks: Vec<_> = cities.into_iter().map(|city| {
        // tokio::spawn membuat "Green Thread" baru. 
        // Backend tidak menunggu BPN selesai baru ke SMD, tapi jalan bareng.
        tokio::spawn(async move {
            fetch_from_specific_city(city, movie_id).await
        })
    }).collect();

    // C. GATHER: Tunggu semua Task selesai bareng-bareng
    let results = join_all(tasks).await;

    // D. Rapikan hasil (Unwrap dari thread result)
    let final_response: Vec<CityScheduleResponse> = results
        .into_iter()
        .filter_map(|res| res.ok()) // Buang task yang panic/crash parah
        .collect();

    Json(final_response)
}

// ==========================================
// 4. WORKER FUNCTION (Dipanggil oleh Thread)
// ==========================================

async fn fetch_from_specific_city(city: CityConfig, movie_id: i64) -> CityScheduleResponse {
    // Mencoba connect ke DB Kota spesifik
    // Kita set timeout pendek (2 detik), kalau DB lemot/mati, kita skip biar user gak nunggu lama.
    let pool_result = MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect(&city.db_url)
        .await;

    match pool_result {
        Ok(pool) => {
            // Query jadwal tayang (SQL standard)
            // Mengambil nama bioskop, studio, jam, dan harga
            let query = r#"
                SELECT 
                    c.name as cinema_name, 
                    s.name as studio_name, 
                    DATE_FORMAT(st.start_time, '%H:%i') as start_time, 
                    st.price
                FROM showtimes st
                JOIN studios s ON st.studio_id = s.id
                JOIN cinemas c ON s.cinema_id = c.id
                WHERE st.global_movie_id = ?
                ORDER BY st.start_time ASC
            "#;

            let schedules = sqlx::query_as::<_, ShowtimeDetail>(query)
                .bind(movie_id)
                .fetch_all(&pool)
                .await
                .unwrap_or_else(|_| vec![]);

            // Tutup koneksi agar tidak menumpuk
            pool.close().await;

            CityScheduleResponse {
                city: city.name,
                schedules,
                status: "Online".to_string(),
            }
        }
        Err(e) => {
            println!("⚠️ Gagal konek ke {}: {}", city.name, e);
            // Fault Tolerance: Return object kosong dengan status Error
            CityScheduleResponse {
                city: city.name,
                schedules: vec![],
                status: "Offline/Error".to_string(),
            }
        }
    }
}
