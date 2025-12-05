-- ==========================================
-- DATABASE CENTRAL (MASTER DATABASE)
-- ==========================================

CREATE DATABASE IF NOT EXISTS tioskop_central_db;
USE tioskop_central_db;

-- 1. TABEL USERS (Pengguna Global)
CREATE TABLE users (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL, -- Diisi hash password
    role ENUM('admin', 'customer') DEFAULT 'customer',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- 2. TABEL MOVIES (Data Master Film)
CREATE TABLE movies (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    poster_url VARCHAR(255), -- Link gambar poster
    duration INT, -- Dalam menit
    description TEXT,
    release_date DATE
);

-- 3. TABEL CITIES (KUNCI MULTIPROCESSING!)
-- Ini adalah "Buku Telepon" bagi Backend Axum untuk tahu harus connect kemana.
CREATE TABLE cities (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,      -- Nama Kota (untuk UI)
    db_name VARCHAR(100) NOT NULL,   -- Nama Database fisik
    db_url VARCHAR(255) NOT NULL,    -- Connection String lengkap (PENTING BUAT RUST)
    is_active BOOLEAN DEFAULT TRUE   -- Kalau FALSE, kota ini diskip (misal lagi maintenance)
);

-- ==========================================
-- DUMMY DATA CENTRAL
-- ==========================================

INSERT INTO users (name, email, password) VALUES 
('Alief Mahasiswa', 'alief@itk.ac.id', 'rahasia123');

INSERT INTO movies (id, title, duration, description, release_date) VALUES 
(1, 'Inception', 148, 'Mimpi dalam mimpi...', '2010-07-16'),
(2, 'Interstellar', 169, 'Perjalanan lubang cacing...', '2014-11-07');

-- KONFIGURASI KONEKSI DATABASE (Sesuaikan user:pass dengan laptopmu!)
-- Format MySQL: mysql://username:password@localhost:port/nama_db
INSERT INTO cities (name, db_name, db_url) VALUES 
('Balikpapan', 'tioskop_balikpapan_db', 'mysql://root:@localhost:3306/tioskop_balikpapan_db'),
('Samarinda', 'tioskop_samarinda_db', 'mysql://root:@localhost:3306/tioskop_samarinda_db');
