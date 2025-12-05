-- Update poster URLs untuk movies yang sudah ada
-- Jalankan di Central DB (tioskop_central_db)

USE tioskop_central_db;

-- Update poster URLs dengan path yang benar
UPDATE movies SET poster_url = '/film-1.webp' WHERE id = 1;
UPDATE movies SET poster_url = '/film-2.webp' WHERE id = 2;

-- Atau jika ingin menggunakan URL lengkap:
-- UPDATE movies SET poster_url = 'http://localhost:5173/film-1.webp' WHERE id = 1;
-- UPDATE movies SET poster_url = 'http://localhost:5173/film-2.webp' WHERE id = 2;

-- Cek hasil
SELECT id, title, poster_url FROM movies;
