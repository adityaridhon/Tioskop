-- ==========================================
-- DATABASE BALIKPAPAN
-- ==========================================

CREATE DATABASE IF NOT EXISTS tioskop_balikpapan_db;
USE tioskop_balikpapan_db;

-- 1. CINEMAS
CREATE TABLE cinemas (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address TEXT
);

-- 2. STUDIOS
CREATE TABLE studios (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    cinema_id BIGINT,
    name VARCHAR(50),
    type VARCHAR(20) DEFAULT 'Regular'
);

-- 3. SHOWTIMES (Jadwal Tayang)
CREATE TABLE showtimes (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    global_movie_id BIGINT NOT NULL, -- ID 1 = Inception (referensi ke Central DB)
    studio_id BIGINT NOT NULL,
    start_time DATETIME NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);

-- 4. BOOKINGS (Transaksi Lokal)
CREATE TABLE local_bookings (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    global_user_id BIGINT NOT NULL, -- ID User dari Central DB
    showtime_id BIGINT NOT NULL,
    status VARCHAR(20) DEFAULT 'PAID'
);

-- ==========================================
-- DUMMY DATA BALIKPAPAN
-- ==========================================

INSERT INTO cinemas (id, name, address) VALUES (1, 'Tioskop E-Walk BPN', 'Jl. Jend Sudirman');
INSERT INTO studios (id, cinema_id, name) VALUES (1, 1, 'Studio 1');

-- Jadwal Film INCEPTION (ID: 1) di Balikpapan
INSERT INTO showtimes (global_movie_id, studio_id, start_time, price) VALUES 
(1, 1, '2025-12-10 14:00:00', 45000), -- Jam Siang
(1, 1, '2025-12-10 16:30:00', 45000); -- Jam Sore
