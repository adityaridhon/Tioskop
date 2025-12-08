# **Tioskop – Lihat & Booking Film 🎫**

_A Functional Programming Approach with Rust_

**Authors:**<br/>
Aditya Ridho Nugroho | Alief Rachmattul Islam | Arya Zaky Pradipta | Muhamad Faisal | Muhammad Fatwa Al Choiri

---

## **Abstract**

Tioskop adalah aplikasi jadwal dan booking bioskop modern yang dibangun menggunakan **Rust** sebagai Backend dan **Vue.js** sebagai Frontend. Sistem ini menerapkan pendekatan _functional programming_ dan arsitektur yang _type-safe_. Backend dikembangkan menggunakan framework **Axum** dan runtime asynchronous **Tokio**, serta memanfaatkan **SeaORM** untuk ORM database dan **Rayon** untuk pemrosesan paralel data yang intensif.

---

## **Introduction**

Aplikasi ini dirancang untuk menyelesaikan permasalahan utama pada sistem jadwal bioskop konvensional:
- Kesulitan dalam menangani _concurrency_ tinggi saat booking tiket populer.
- Kurangnya integrasi data yang aman dan cepat antara jadwal dan ketersediaan kursi.

### Mengapa Rust?

| Alasan              | Penjelasan                                               |
| ------------------- | -------------------------------------------------------- |
| **Memory Safety** | Mencegah _null pointer_ dan _race conditions_ secara compile-time. |
| **High Concurrency**| Runtime Tokio memungkinkan ribuan request async tanpa _overhead_ thread OS yang besar. |
| **Type Safety** | Sistem tipe Rust menjamin integritas data dari API hingga Database. |

### Tujuan Utama

- Memberikan sistem manajemen bioskop yang cepat, scalable, dan aman.
- Mengaplikasikan paradigma **Functional Programming** (Immutability, Pure Functions) dan **Parallel Processing**.
- Menyediakan pengalaman pengguna yang responsif dengan Single Page Application (SPA).

---

## **Background & Concepts**

### Technology Stack

| Komponen           | Teknologi                                      |
| ------------------ | ---------------------------------------------- |
| **Backend** | Rust + Axum                                    |
| **Frontend** | Vue.js 3 + TailwindCSS (Vite Build Tool)       |
| **Database & ORM** | MySQL + SeaORM (Async ORM)                     |
| **Runtime** | Tokio (Async)                                  |
| **Parallelism** | Rayon (Data Parallelism)                       |
| **Authentication** | JWT (JSON Web Token) + Argon2 (Hashing)        |

### Konsep Functional Programming & Advanced Features

| Konsep           | Implementasi Dalam Proyek                                       |
| ---------------- | --------------------------------------------------------------- |
| **Pure Function**| Transformasi DTO (Data Transfer Object) dan validasi logika bisnis. |
| **Immutability** | Penggunaan `Reference Counter (Arc)` untuk state sharing yang aman. |
| **Concurrency** | `Tokio::spawn` untuk task async dan `Rayon` untuk kalkulasi berat (misal: reporting/workflow). |
| **Middleware** | Custom `AuthMiddleware` untuk proteksi route berbasis JWT. |

---

## **Source Code Overview**

### Struktur Folder Backend

Struktur backend telah direfaktor untuk memisahkan _concerns_ antara Controller (Handler), Business Logic (Service), dan Data Access (Entity).
