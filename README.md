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
| Frontend           | **VueJS**       |
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

Struktur API utama:

| Endpoint   | Fungsi                       |
| ---------- | ---------------------------- |
| /movies    | daftar film                  |
| /showtimes | jadwal film per studio       |
| /studios   | daftar studio & jumlah kursi |
| /book      | booking tiket                |

Fungsi booking mengikuti rules:

1. User memilih film → showtime → kursi.
2. Sistem mengecek ketersediaan seat.
3. Jika tersedia → insert booking.
4. Kursi yang sudah terbooking akan ditolak request baru.

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
