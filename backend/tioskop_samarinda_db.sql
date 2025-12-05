-- ==========================================
-- DATABASE SAMARINDA
-- ==========================================

CREATE DATABASE IF NOT EXISTS tioskop_samarinda_db;
USE tioskop_samarinda_db;

-- Struktur Tabel SAMA PERSIS dengan Balikpapan
CREATE TABLE cinemas (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    address TEXT
);

CREATE TABLE studios (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    cinema_id BIGINT,
    name VARCHAR(50),
    type VARCHAR(20) DEFAULT 'Regular'
);

CREATE TABLE showtimes (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    global_movie_id BIGINT NOT NULL, 
    studio_id BIGINT NOT NULL,
    start_time DATETIME NOT NULL,
    price DECIMAL(10, 2) NOT NULL
);

CREATE TABLE local_bookings (
    id BIGINT AUTO_INCREMENT PRIMARY KEY,
    global_user_id BIGINT NOT NULL,
    showtime_id BIGINT NOT NULL,
    status VARCHAR(20) DEFAULT 'PAID'
);

-- ==========================================
-- DUMMY DATA SAMARINDA
-- ==========================================

INSERT INTO cinemas (id, name, address) VALUES (1, 'Tioskop BigMall SMD', 'Jl. Untung Suropati');
INSERT INTO studios (id, cinema_id, name, type) VALUES (1, 1, 'IMAX Studio', 'IMAX');

-- Jadwal Film INCEPTION (ID: 1) di Samarinda (Hanya malam, harga mahal)
INSERT INTO showtimes (global_movie_id, studio_id, start_time, price) VALUES 
(1, 1, '2025-12-10 19:00:00', 85000); -- Jam Malam & Mahal
