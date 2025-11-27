# Dokumentasi API - Tioskop Backend

Dokumentasi REST API untuk sistem lihat dan booking bioskop.

---

## 🌐 Informasi Umum

### Base URL

```
http://127.0.0.1:3000
```

### Response Format

Semua endpoint menggunakan format response:

**Success Response:**

```json
{
  "success": true,
  "message": "Pesan sukses",
  "data": {
    /* Isi data */
  }
}
```

**Error Response:**

```json
{
  "success": false,
  "message": "Pesan error",
  "data": null
}
```

### HTTP Status Codes

| Status Code                 | Deskripsi                |
| --------------------------- | ------------------------ |
| `200 OK`                    | Request berhasil         |
| `201 Created`               | Resource berhasil dibuat |
| `400 Bad Request`           | Request tidak valid      |
| `404 Not Found`             | Resource tidak ditemukan |
| `500 Internal Server Error` | Terjadi error di server  |

---

## 🎬 Movies API

Endpoint untuk mengelola data film.

### 1. Get All Movies

Mengambil semua film dari database.

**Endpoint:**

```
GET /api/movies/all
```

**Response Success (200 OK):**

```json
{
  "success": true,
  "message": "Berhasil mengambil semua film",
  "data": [
    {
      "id": 1,
      "title": "Deadpool & Wolverine",
      "genre": "Action, Comedy",
      "rating": "8.4",
      "duration": 130,
      "description": "Kolaborasi anti-hero Marvel penuh humor brutal",
      "poster_url": "poster1.jpg",
      "release_date": "2024-09-21"
    }
  ]
}
```

---

### 2. Search Movies

Mencari film berdasarkan judul (case-insensitive).

**Endpoint:**

```
GET /api/movies?q={search_query}
```

**Query Parameters:**

- `q` (optional): Kata kunci pencarian

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mencari film",
  "data": [
    {
      "id": 1,
      "title": "Deadpool & Wolverine",
      "genre": "Action, Comedy",
      "rating": "8.4",
      "duration": 130,
      "description": "Kolaborasi anti-hero Marvel penuh humor brutal",
      "poster_url": "poster1.jpg",
      "release_date": "2024-09-21"
    }
  ]
}
```

---

### 3. Create Movie

Menambahkan film baru ke database.

**Endpoint:**

```
POST /api/movies
```

**Request Body:**

```json
{
  "title": "Inception",
  "genre": "Sci-Fi, Thriller",
  "rating": "8.8",
  "duration": 148,
  "description": "A thief who steals corporate secrets through dream-sharing technology",
  "poster_url": "inception.jpg",
  "release_date": "2010-07-16"
}
```

**Field Requirements:**

- `title` (string, required): Judul film
- `genre` (string, required): Genre film
- `rating` (string, required): Rating film (contoh: "8.5")
- `duration` (number, required): Durasi dalam menit
- `description` (string, optional): Deskripsi film
- `poster_url` (string, optional): URL poster
- `release_date` (string, required): Tanggal rilis (format: YYYY-MM-DD)

**Response Success (201 Created):**

```json
{
  "success": true,
  "message": "Berhasil menambahkan film",
  "data": {
    "id": 5,
    "title": "Inception",
    "genre": "Sci-Fi, Thriller",
    "rating": "8.8",
    "duration": 148,
    "description": "A thief who steals corporate secrets through dream-sharing technology",
    "poster_url": "inception.jpg",
    "release_date": "2010-07-16"
  }
}
```

---

### 4. Update Movie

Mengupdate data film berdasarkan ID.

**Endpoint:** `PUT /api/movies/{id}`

**Path Parameters:**

- `id`: ID film yang akan diupdate

**Request Body:**

```json
{
  "title": "Inception (Updated)",
  "rating": "9.0"
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengupdate film",
  "data": {
    "id": 5,
    "title": "Inception (Updated)",
    "genre": "Sci-Fi, Thriller",
    "rating": "9.0",
    "duration": 148,
    "description": "A thief who steals corporate secrets through dream-sharing technology",
    "poster_url": "inception.jpg",
    "release_date": "2010-07-16"
  }
}
```

---

### 5. Delete Movie

Menghapus film berdasarkan ID.

**Endpoint:** `DELETE /api/movies/{id}`

**Path Parameters:**

- `id`: ID film yang akan dihapus

**Response:**

```json
{
  "success": true,
  "message": "Berhasil menghapus film",
  "data": {
    "id": 5,
    "deleted": true
  }
}
```

---

## Showtimes API

### 1. Get All Showtimes

Mengambil semua jadwal tayang.

**Endpoint:** `GET /api/showtimes`

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil semua showtimes",
  "data": [
    {
      "id": 1,
      "movie_id": 1,
      "studio_id": 1,
      "start_time": "2025-11-27T14:00:00",
      "price": "55000.00"
    }
  ]
}
```

---

### 2. Get Showtimes by Movie

Mengambil jadwal tayang berdasarkan film.

**Endpoint:** `GET /api/showtimes/movie/{movie_id}`

**Path Parameters:**

- `movie_id`: ID film

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil showtimes untuk film ini",
  "data": [
    {
      "id": 1,
      "movie_id": 1,
      "studio_id": 1,
      "start_time": "2025-11-27T14:00:00",
      "price": "55000.00"
    }
  ]
}
```

---

### 3. Create Showtime

Menambahkan jadwal tayang baru.

**Endpoint:** `POST /api/showtimes`

**Request Body:**

```json
{
  "movie_id": 1,
  "studio_id": 2,
  "start_time": "2025-11-28T19:00:00",
  "price": 60000.0
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil menambahkan showtime",
  "data": {
    "id": 10,
    "movie_id": 1,
    "studio_id": 2,
    "start_time": "2025-11-28T19:00:00",
    "price": "60000.00"
  }
}
```

---

### 4. Update Showtime

Mengupdate jadwal tayang.

**Endpoint:** `PUT /api/showtimes/{id}`

**Path Parameters:**

- `id`: ID showtime

**Request Body:**

```json
{
  "price": 65000.0,
  "start_time": "2025-11-28T20:00:00"
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengupdate showtime",
  "data": {
    "id": 10,
    "movie_id": 1,
    "studio_id": 2,
    "start_time": "2025-11-28T20:00:00",
    "price": "65000.00"
  }
}
```

---

### 5. Delete Showtime

Menghapus jadwal tayang.

**Endpoint:** `DELETE /api/showtimes/{id}`

**Path Parameters:**

- `id`: ID showtime

**Response:**

```json
{
  "success": true,
  "message": "Berhasil menghapus showtime",
  "data": {
    "id": 10,
    "deleted": true
  }
}
```

---

## Studios API

### 1. Get All Studios

Mengambil semua studio.

**Endpoint:** `GET /api/studios`

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil semua studio",
  "data": [
    {
      "id": 1,
      "cinema_id": 1,
      "name": "Studio 1",
      "capacity": 100,
      "type": "Regular"
    }
  ]
}
```

---

### 2. Get Studio by ID

Mengambil studio berdasarkan ID.

**Endpoint:** `GET /api/studios/{id}`

**Path Parameters:**

- `id`: ID studio

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil studio",
  "data": {
    "id": 1,
    "cinema_id": 1,
    "name": "Studio 1",
    "capacity": 100,
    "type": "Regular"
  }
}
```

---

### 3. Get Studios by Cinema

Mengambil studio berdasarkan cinema.

**Endpoint:** `GET /api/studios/cinema/{cinema_id}`

**Path Parameters:**

- `cinema_id`: ID cinema

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil studio untuk cinema ini",
  "data": [
    {
      "id": 1,
      "cinema_id": 1,
      "name": "Studio 1",
      "capacity": 100,
      "type": "Regular"
    }
  ]
}
```

---

### 4. Create Studio

Menambahkan studio baru.

**Endpoint:** `POST /api/studios`

**Request Body:**

```json
{
  "cinema_id": 1,
  "name": "Studio IMAX",
  "capacity": 200,
  "type": "IMAX"
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil menambahkan studio",
  "data": {
    "id": 5,
    "cinema_id": 1,
    "name": "Studio IMAX",
    "capacity": 200,
    "type": "IMAX"
  }
}
```

---

### 5. Update Studio

Mengupdate data studio.

**Endpoint:** `PUT /api/studios/{id}`

**Path Parameters:**

- `id`: ID studio

**Request Body:**

```json
{
  "name": "Studio IMAX Premium",
  "capacity": 250
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengupdate studio",
  "data": {
    "id": 5,
    "cinema_id": 1,
    "name": "Studio IMAX Premium",
    "capacity": 250,
    "type": "IMAX"
  }
}
```

---

### 6. Delete Studio

Menghapus studio.

**Endpoint:** `DELETE /api/studios/{id}`

**Path Parameters:**

- `id`: ID studio

**Response:**

```json
{
  "success": true,
  "message": "Berhasil menghapus studio",
  "data": {
    "id": 5,
    "deleted": true
  }
}
```

---

## Seats API

### 1. Get Seats by Studio

Mengambil semua kursi di studio tertentu.

**Endpoint:** `GET /api/seats/studio/{studio_id}`

**Path Parameters:**

- `studio_id`: ID studio

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil kursi untuk studio ini",
  "data": [
    {
      "id": 1,
      "studio_id": 1,
      "seat_code": "A1",
      "seat_row": "A",
      "seat_col": 1,
      "seat_status": "available"
    }
  ]
}
```

---

### 2. Get Available Seats by Showtime

Mengambil kursi yang tersedia untuk jadwal tayang tertentu.

**Endpoint:** `GET /api/seats/showtime/{showtime_id}/available`

**Path Parameters:**

- `showtime_id`: ID showtime

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil kursi tersedia untuk showtime ini",
  "data": [
    {
      "id": 1,
      "studio_id": 1,
      "seat_code": "A1",
      "seat_row": "A",
      "seat_col": 1,
      "seat_status": "available"
    }
  ]
}
```

---

### 3. Generate Seats for Studio

Generate kursi otomatis untuk studio (A1-A10, B1-B10, dst).

**Endpoint:** `POST /api/seats/generate`

**Request Body:**

```json
{
  "studio_id": 2,
  "rows": 8,
  "seats_per_row": 10
}
```

**Response:**

```json
{
  "success": true,
  "message": "Berhasil generate 80 kursi untuk studio 2",
  "data": {
    "studio_id": 2,
    "total_seats_created": 80
  }
}
```

---

## Bookings API

Endpoint untuk mengelola booking tiket. Mendukung **multiple seats** per booking.

### 1. Get All Bookings

Mengambil semua booking dengan detail kursi.

**Endpoint:**

```
GET /api/bookings
```

**Response Success (200 OK):**

```json
{
  "success": true,
  "message": "Berhasil mengambil semua booking",
  "data": [
    {
      "id": 1,
      "user_id": 1,
      "showtime_id": 1,
      "booking_code": "BK001",
      "total_price": "165000.00",
      "payment_status": "PAID",
      "created_at": "2025-11-27T10:30:00",
      "seats": [
        {
          "seat_id": 15,
          "seat_code": "A5",
          "price": "55000.00"
        },
        {
          "seat_id": 16,
          "seat_code": "A6",
          "price": "55000.00"
        },
        {
          "seat_id": 17,
          "seat_code": "A7",
          "price": "55000.00"
        }
      ]
    }
  ]
}
```

---

### 2. Get Booking by ID

Mengambil detail booking berdasarkan ID dengan informasi kursi.

**Endpoint:**

```
GET /api/bookings/{id}
```

**Path Parameters:**

- `id` (number): ID booking

**Response Success (200 OK):**

```json
{
  "success": true,
  "message": "Berhasil mengambil booking",
  "data": {
    "id": 1,
    "user_id": 1,
    "showtime_id": 1,
    "booking_code": "BK001",
    "total_price": "165000.00",
    "payment_status": "PAID",
    "created_at": "2025-11-27T10:30:00",
    "seats": [
      {
        "seat_id": 15,
        "seat_code": "A5",
        "price": "55000.00"
      },
      {
        "seat_id": 16,
        "seat_code": "A6",
        "price": "55000.00"
      }
    ]
  }
}
```

**Response Error (404 Not Found):**

```json
{
  "success": false,
  "message": "Booking tidak ditemukan",
  "data": null
}
```

---

### 3. Get Bookings by User

Mengambil semua booking milik user tertentu.

**Endpoint:** `GET /api/bookings/user/{user_id}`

**Path Parameters:**

- `user_id`: ID user

**Response:**

```json
{
  "success": true,
  "message": "Berhasil mengambil booking untuk user ini",
  "data": [
    {
      "id": 1,
      "user_id": 1,
      "showtime_id": 1,
      "seat_id": 15,
      "booking_time": "2025-11-27T10:30:00",
      "total_price": "55000.00",
      "payment_status": "paid",
      "payment_method": "credit_card"
    }
  ]
}
```

---

### 4. Create Booking

Membuat booking baru untuk **satu atau lebih kursi**.

**Endpoint:**

```
POST /api/bookings
```

**Request Body:**

```json
{
  "user_id": 2,
  "showtime_id": 3,
  "seat_ids": [15, 16, 17]
}
```

**Field Requirements:**

- `user_id` (number, required): ID user yang melakukan booking
- `showtime_id` (number, required): ID jadwal tayang
- `seat_ids` (array of numbers, required): Array ID kursi yang dipesan

**Business Logic:**

- Otomatis generate `booking_code` unik
- Hitung `total_price` dari jumlah kursi × harga showtime
- Set status pembayaran ke `PENDING`
- Validasi kursi tersedia untuk showtime yang dipilih
- Insert data ke tabel `bookings` dan `booking_seats`

**Response Success (201 Created):**

```json
{
  "success": true,
  "message": "Berhasil membuat booking",
  "data": {
    "id": 10,
    "user_id": 2,
    "showtime_id": 3,
    "booking_code": "BK010",
    "total_price": "165000.00",
    "payment_status": "PENDING",
    "created_at": "2025-11-27T15:45:00",
    "seats": [
      {
        "seat_id": 15,
        "seat_code": "A5",
        "price": "55000.00"
      },
      {
        "seat_id": 16,
        "seat_code": "A6",
        "price": "55000.00"
      },
      {
        "seat_id": 17,
        "seat_code": "A7",
        "price": "55000.00"
      }
    ]
  }
}
```

**Response Error - Kursi tidak tersedia (400 Bad Request):**

```json
{
  "success": false,
  "message": "Kursi dengan ID 15 sudah di-booking untuk showtime ini",
  "data": null
}
```

---

### 5. Update Payment Status

Mengupdate status pembayaran booking (untuk konfirmasi pembayaran).

**Endpoint:**

```

PUT /api/bookings/{id}/payment

```

**Path Parameters:**

- `id` (number): ID booking

**Request Body:**

```json
{
  "payment_status": "PAID"
}
```

**Allowed Values:**

- `PENDING`: Menunggu pembayaran
- `PAID`: Sudah dibayar
- `CANCELLED`: Dibatalkan

**Response Success (200 OK):**

```json
{
  "success": true,
  "message": "Berhasil mengupdate status pembayaran",
  "data": {
    "id": 10,
    "user_id": 2,
    "showtime_id": 3,
    "booking_code": "BK010",
    "total_price": "165000.00",
    "payment_status": "PAID",
    "created_at": "2025-11-27T15:45:00",
    "seats": [
      {
        "seat_id": 15,
        "seat_code": "A5",
        "price": "55000.00"
      }
    ]
  }
}
```

---

### 6. Cancel Booking

Membatalkan booking dan ubah status menjadi `CANCELLED`.

**Endpoint:**

```
PUT /api/bookings/{id}/cancel
```

**Path Parameters:**

- `id` (number): ID booking

**Response Success (200 OK):**

```json
{
  "success": true,
  "message": "Berhasil membatalkan booking",
  "data": {
    "id": 10,
    "user_id": 2,
    "showtime_id": 3,
    "booking_code": "BK010",
    "total_price": "165000.00",
    "payment_status": "CANCELLED",
    "created_at": "2025-11-27T15:45:00",
    "seats": [
      {
        "seat_id": 15,
        "seat_code": "A5",
        "price": "55000.00"
      }
    ]
  }
}
```

---

## ⚠️ Error Handling

### Error Response Format

Semua error menggunakan format standar:

```json
{
  "success": false,
  "message": "Deskripsi error",
  "data": null
}
```

### Common Errors

**1. Resource Not Found (404)**

```json
{
  "success": false,
  "message": "Film dengan id 999 tidak ditemukan",
  "data": null
}
```

**2. Validation Error (400)**

```json
{
  "success": false,
  "message": "Field 'title' wajib diisi",
  "data": null
}
```

**3. Duplicate Booking (400)**

```json
{
  "success": false,
  "message": "Kursi dengan ID 15 sudah di-booking untuk showtime ini",
  "data": null
}
```

**4. Database Error (500)**

```json
{
  "success": false,
  "message": "Database error: [detail error]",
  "data": null
}
```

---

## Functional Programming Concepts

Semua endpoint diimplementasikan dengan:

### 1. **Immutability**

- Tidak ada mutasi data
- Semua transformasi menghasilkan value baru

### 2. **Higher-Order Functions**

```rust
query.fetch_all(&pool)
    .await
    .map(|movies| ApiResponse::success("Berhasil", movies))
    .unwrap_or_else(|e| ApiResponse::error(&format!("Error: {}", e)))
```

### 3. ** Function Composition**

- Pipeline transformasi data: `fetch → map → unwrap_or_else`
- Chaining operations tanpa intermediate variables

### 4. **Pattern Matching**

```rust
match sqlx::query_as::<_, Booking>("...").fetch_one(&pool).await {
    Ok(booking) => ApiResponse::success("Berhasil", booking),
    Err(e) => ApiResponse::error(&format!("Error: {}", e)),
}
```

### 5. **Pure Functions**

- Handler functions tidak memiliki side effects
- Deterministic: input sama → output sama

### 6. **Iterators & Closures**

```rust
seat_ids.iter()
    .map(|&seat_id| /* transform */)
    .collect()
```

---

## 🔧 Tech Stack

- **Language**: Rust (Edition 2024)
- **Framework**: Axum 0.8.7
- **Database**: MySQL with SQLx 0.8.6
- **Runtime**: Tokio 1.48.0 (async/await)
- **Serialization**: Serde 1.0.228
- **CORS**: Tower-http 0.6.6

---

## 📝 Notes

1. Semua endpoint menggunakan **async/await** dengan Tokio runtime
2. Connection pooling otomatis (max 10 connections)
3. CORS diaktifkan untuk development
4. Timestamp database menggunakan `DATETIME` format
5. Decimal handling menggunakan `rust_decimal` untuk presisi

---
