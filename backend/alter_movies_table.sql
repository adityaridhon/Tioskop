-- ALTER TABLE untuk menambahkan kolom genre dan rating ke tabel movies
-- Jalankan di Central DB (tioskop_central_db)

USE tioskop_central_db;

ALTER TABLE movies 
ADD COLUMN genre VARCHAR(100) AFTER title,
ADD COLUMN rating VARCHAR(10) AFTER genre;

-- Update existing data dengan nilai default (optional)
UPDATE movies SET genre = 'Drama' WHERE genre IS NULL;
UPDATE movies SET rating = 'PG-13' WHERE rating IS NULL;
