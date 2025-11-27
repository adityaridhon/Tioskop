# **Tioskop – Lihat & Booking Film 🎫**

_A Functional Programming Approach with Rust_

**Authors:**<br/>
Aditya Ridho Nugroho | Alief Rachmattul Islam | Arya Zaky Pradipta | Muhamad Faisal | Muhammad Fatwa Al-Choiri

---

## **Abstract**

Tioskop adalah aplikasi jadwal dan booking bioskop yang dibangun menggunakan Rust sebagai Backend dan Vue.js sebagai Frontend dengan pendekatan _functional programming_. Backend dikembangkan menggunakan framework **Axum** dan runtime asynchronous **Tokio**, memungkinkan sistem menangani request secara _concurrent_, _async_.

---

## **Introduction**

Aplikasi ini dirancang untuk menyelesaikan permasalahan utama pada sistem jadwal bioskop umumnya yatiu:

- Pembatasan informasi jadwal yang tidak update dan lambat.
- Dibutuhkan sistem modern dengan arsitektur aman, efisien, dan scalable.

### Mengapa Rust?

| Alasan              | Penjelasan                                               |
| ------------------- | -------------------------------------------------------- |
| Efisiensi memory    | Mengurangi crash pada booking concurrency.               |
| High concurrency    | Cocok untuk sistem jadwal & booking yang banyak request. |
| Functional friendly | Mendukung paradigma pemrograman fungsional.              |

### Tujuan Utama

- Memberikan sistem manajemen bioskop yang cepat, scalable, dan aman.
- Menyediakan API lihat dan booking film yang cepat dan tepat.
- Mengaplikasikan paradigma **Functional Programming** dalam implementasi pengembangan sistem.

---

## **Background & Concepts**

### Technology Stack

| Komponen           | Teknologi       |
| ------------------ | --------------- |
| Backend            | **Rust + Axum** |
| Runtime Async      | **Tokio**       |
| Database           | MySQL           |
| JSON Serialization | Serde           |

### Konsep Functional Programming Dalam Sistem

| Konsep FP        | Implementasi Dalam Proyek                                       |
| ---------------- | --------------------------------------------------------------- |
| Pure Function    | Perhitungan harga, validasi seat, transformasi data API         |
| Immutability     | State kursi tidak berubah, booking dicatat sebagai _event baru_ |
| Pattern Matching | Handling error + branch booking logic                           |

Dengan ini aplikasi bisa menangani ratusan request booking serentak tanpa konflik seat.

---

## **Source Code Overview**

### Struktur Folder Backend

```
backend/
├── src/
│   ├── main.rs
│   │
│   ├── config/
│   │   └── mod.rs                   # Database konfigurasi
│   │
│   ├── models/                      # Data model
│   │   ├── mod.rs
│   │   ├── movie.rs
│   │   ├── showtime.rs
│   │   ├── studio.rs
│   │   ├── seat.rs
│   │   ├── booking.rs
│   │   └── response.rs
│   │
│   ├── handlers/                    # Logic
│   │   ├── mod.rs
│   │   ├── movie_handler.rs
│   │   ├── showtime_handler.rs
│   │   ├── studio_handler.rs
│   │   ├── seat_handler.rs
│   │   └── booking_handler.rs
│   │
│   └── routes/                      # Routing endpoint
│       ├── mod.rs
│       ├── movie_routes.rs
│       ├── showtime_routes.rs
│       ├── studio_routes.rs
│       ├── seat_routes.rs
│       └── booking_routes.rs
│
├── Cargo.toml                       # Dependencies
├── .env                             # Environment variables
└── tioskop_db.sql                   # Database schema
```

### File Utama

#### **src/main.rs**

Main point aplikasi yang menjalankan:

- Tokio async runtime menggunakan `#[tokio::main]`
- Database connection
- CORS middleware configuration untuk devlepoment
- Router dari semua module
- Jalankan Server di `127.0.0.1:3000`

**SC:**

---

#### **src/config/mod.rs**

Konfigurasi database connection pool menggunakan SQLx:

- Database URL dari environment variable
- Max connections: 10 concurrent connections
- MySQL connection pooling

**SC:**

---

### Models Layer

#### **src/models/response.rs**

Membuat response wrapper untuk semua response API endpoints dengan kondisi succes dan error:

**SC:**

---

#### **src/models/movie.rs**

Membuat model movie:

dengan ket:

- `Movie`: Database model
- `CreateMovieRequest`: untuk Create Request
- `UpdateMovieRequest`: untuk Update Request
- `SearchParams`: Query parameters untuk pencarian film

**Fields:**

- `id`, `title`, `genre`, `rating`, `duration`, `description`, `poster_url`, `release_date`

---

#### **src/models/showtime.rs**

Membuat model Showtimes:

#### **src/models/studio.rs**

Membuat model untuk Studios:

dengan ket:

- `Studio`: Database model
- `CreateStudioRequest`: untuk Create Request
- `UpdateStudioRequest`: untuk Update Request

**Fields:**

- `id`, `cinema_id`, `name`, `capacity`, `type`

#### **src/models/seat.rs**

Membuat model untuk Seats:

dengan ket:

- `Seat`: Database model untuk kursi
- `SeatWithBookingStatus`: Extended model dengan status booking
- `GenerateSeatsRequest`: Fungsi untuk auto-generate kursi

**Fields:**

- `id`, `studio_id`, `seat_code`, `seat_row`, `seat_col`, `seat_status`

#### **src/models/booking.rs**

Membuat model untuk Bookings:

dengan ket:

- `Booking`: Database model
- `BookingSeat`: Relasi booking dengan seat
- `CreateBookingRequest`: Fungsi booking dengan banyak kursi
- `UpdatePaymentStatusRequest`: Fngsi untuk update payment
- `BookingDetail`: Response detail book
- `BookingSeatDetail`: Detail kursi dalam booking

**Fields:**

- `id`, `user_id`, `showtime_id`, `booking_code`, `total_price`, `payment_status`, `created_at`

---

### Handlers Layer (Business Logic)

Semua handlers diimplementasikan dengan:

- `.map()` untuk transformasi data
- `.unwrap_or_else()` untuk error handling
- Pattern matching dengan `match`
- Immutable transformations

#### **src/handlers/movie_handler.rs**

CRUD operations untuk Movies:

**Functions:**

- `get_all_movies()`: Fetch semua film
- `search_movies()`: Search dengan query parameter
- `create_movie()`: Insert film baru
- `update_movie()`: Update partial fields
- `delete_movie()`: Delete film

#### **src/handlers/showtime_handler.rs**

CRUD operations untuk Showtimes:

**Functions:**

- `get_all_showtimes()`: Fetch semua jadwal
- `get_showtimes_by_movie()`: Filter by movie_id
- `create_showtime()`: Insert jadwal baru
- `update_showtime()`: Update jadwal
- `delete_showtime()`: Delete jadwal

#### **src/handlers/studio_handler.rs**

CRUD operations untuk Studios:

**Functions:**

- `get_all_studios()`: Fetch semua studio
- `get_studio_by_id()`: Get single studio
- `get_studios_by_cinema()`: Filter by cinema_id
- `create_studio()`: Insert studio baru
- `update_studio()`: Update studio data
- `delete_studio()`: Delete studio

#### **src/handlers/seat_handler.rs**

Operations untuk Seats dengan generator:

**Functions:**

- `get_seats_by_studio()`: Fetch kursi per studio
- `get_seats_by_showtime()`: Fetch kursi untuk showtime tertentu
- `get_available_seats()`: Filter hanya kursi available
- `generate_seats_for_studio()`: **Auto-generate** kursi (A1-A10, B1-B10, dst)

#### **src/handlers/booking_handler.rs**

Operations untuk Bookings:

**Functions:**

- `get_all_bookings()`: Fetch semua booking dengan detail seats
- `get_booking_by_id()`: Get single booking + seats (JOIN query)
- `get_bookings_by_user()`: Filter by user_id
- `create_booking()`: **Multi-seat booking** dengan validasi
- `update_payment_status()`: Update PENDING → PAID → CANCELLED
- `cancel_booking()`: Cancel booking + kembalikan seat status

---

### Routes Layer (Endpoint Definitions)

#### **src/routes/movie_routes.rs**

#### **src/routes/showtime_routes.rs**

#### **src/routes/studio_routes.rs**

#### **src/routes/seat_routes.rs**

#### **src/routes/booking_routes.rs**

---

## **Screenshot**

OTW

| Tampilan                  | Status |
| ------------------------- | ------ |
| API Get Movies            | OTW    |
| Daftar Studio + Kursi     | OTW    |
| Halaman Booking           | OTW    |
| Response JSON Book Sukses | OTW    |

---

## **Conclusion**

Projek ini menunjukkan bahwa Rust dapat digunakan secara efektif untuk membangun layanan booking bioskop yang memilki kebutuhan:

- Cepat & aman pada sistem concurrency yang tinggi
- Menerapkan paradigma _Functional Programming_ dengan sesuai
- Memiliki integritas data kuat melalui sistem booking atomic

Ke depannya, fitur projek ini dapat dikembangkan menjadi:

- Payment gateway integration
- Notifikasi tiket digital
- Recomender system film berbasis preferences user

---
