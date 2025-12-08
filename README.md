# **Tioskop – Lihat & Booking Film 🎫**

### https://github.com/adityaridhon/Tioskop

_A Functional Programming & Parallel Processing Approach with Rust_

**Authors:**<br/>
Kelompok 3 - Pemrograman Fungsional A <br/>
Aditya Ridho Nugroho | Alief Rachmattul Islam | Arya Zaky Pradipta | Muhamad Faisal | Muhammad Fatwa Al Choiri

---

## **Abstract**

Tioskop adalah aplikasi jadwal dan booking bioskop modern yang dibangun menggunakan **Rust** sebagai Backend dan **Vue.js** sebagai Frontend. Sistem ini menerapkan pendekatan _functional programming_ dan arsitektur yang _type-safe_. Backend dikembangkan menggunakan framework **Axum** dan runtime asynchronous **Tokio**, serta kini telah ditingkatkan dengan **SeaORM** untuk ORM database dan **Rayon** untuk pemrosesan paralel data jadwal yang intensif.

---

## **Introduction**

Aplikasi ini dirancang untuk menyelesaikan permasalahan utama pada sistem jadwal bioskop konvensional:

- Kesulitan dalam menangani _concurrency_ tinggi saat booking tiket populer.
- Kebutuhan akan analisis data (seperti mencari jadwal mendesak) yang cepat dan efisien pada dataset besar.

### Mengapa Rust?

| Alasan              | Penjelasan                                               |
| ------------------- | -------------------------------------------------------- |
| **Memory Safety** | Mencegah _null pointer_ dan _race conditions_ secara compile-time. |
| **High Concurrency**| Runtime Tokio memungkinkan ribuan request async tanpa _overhead_ thread OS yang besar. |
| **Data Parallelism**| Library **Rayon** memungkinkan pemrosesan data (filter/map/reduce) secara paralel di multi-core CPU. |
| **Type Safety** | Sistem tipe Rust (termasuk Enum) menjamin integritas data dari API hingga Database. |

### Tujuan Utama

- Memberikan sistem manajemen bioskop yang cepat, scalable, dan aman.
- Mengaplikasikan paradigma **Functional Programming** (Immutability, Pure Functions) dan **Parallel Processing**.
- Menyediakan API analisis jadwal (_workflow_) yang mampu memproses dataset besar secara efisien.

---

## **Background & Concepts**

### Technology Stack

| Komponen           | Teknologi                                      |
| ------------------ | ---------------------------------------------- |
| Backend            | **Rust + Axum** |
| Frontend           | **Vue.js 3 + TailwindCSS** (Vite Build Tool)   |
| Database ORM       | **SeaORM** (Async & Type-safe)                 |
| Parallel Processing| **Rayon** |
| Runtime Async      | **Tokio** |
| Database           | MySQL                                          |
| Numeric & Decimal  | rust_decimal                                   |
| JSON Serialization | Serde                                          |



### Konsep Functional Programming & Advanced Features

| Konsep FP        | Implementasi Dalam Proyek (Terbaru)                     |
| ---------------- | ------------------------------------------------------- |
| **Pure Function**| Transformasi data di `workflow_service`, kalkulasi statistik tanpa side-effect. |
| **Immutability** | Penggunaan immutable references (`&[Showtime]`) dan `Arc` untuk sharing state yang aman. |
| **Parallel Iterators** | Menggunakan `.par_iter()` dari Rayon untuk filtering dan mapping data jadwal secara paralel. |
| **Enum-First Design**| Return value menggunakan Enum (`HasilAnalisa`, `StatusJadwal`) untuk handling state yang eksplisit. |

---

## **Source Code Overview**

### Struktur Folder Project

Struktur backend telah direfaktor untuk memisahkan _concerns_ antara Controller (Handlers/Routes), Business Logic (Services), dan Data Access (Entities).

---

### File Utama Backend

#### **src/main.rs**

Entry point kini menginisialisasi koneksi **SeaORM** dan **Workflow Service**.

**SC:**

```rust
001  // ... imports
038  #[tokio::main]
039  async fn main() {
040      dotenv().ok();
041
042      // Setup SeaORM database connection
043      let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
044      let db_connection = Database::connect(&database_url).await.expect("Failed to connect");
045
046      // Create workflow service wrapped in Arc for thread-safety
047      let workflow_service = Arc::new(JadwalWorkflowService::new(db_connection.clone()));
048      let workflow_state = WorkflowAppState { workflow_service: workflow_service.clone() };
049
050      // ... Setup Router merging legacy routes and new workflow routes
062      let app = Router::new()
063          .merge(app_routes)
064          .merge(workflow_router) // Nesting /api/workflow
065          .layer(cors);
066
067      // Run server
068      let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
069      axum::serve(listener, app).await.unwrap();
070  }
