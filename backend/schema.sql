
CREATE DATABASE IF NOT EXISTS tioskop_db;
USE tioskop_db;


-- lalu jalankan isi dari file backend/schema.sql
CREATE TABLE IF NOT EXISTS movies (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    poster_url VARCHAR(255),
    rating FLOAT,
    year INT
);

-- Masukkan 8 film dengan poster
INSERT INTO movies (title, description, poster_url, rating, year) VALUES 
('Pesugihan Sate Gagak', 'Tiga sahabat tergiur kekayaan lewat pesugihan sate gagak dan harus menghadapi teror makhluk gaib.', '/src/assets/film-1.webp', 7.5, 2025),
('Pangku', 'Pria desa terjebak hubungan rumit dengan perempuan muda yang menggoyahkan keluarganya.', '/src/assets/film-2.webp', 7.2, 2025),
('Dopamin', 'Sepasang kekasih menemukan koper uang dan terjerumus dalam ketagihan bahaya yang mengancam kewarasan.', '/src/assets/film-3.webp', 7.8, 2025),
('Danyang Wingit: Jumat Kliwon', 'Seorang perempuan terjerat ritual pemanggilan danyang dan kutukan mematikan setiap Jumat Kliwon.', '/src/assets/film-4.webp', 7.3, 2025),
('Wicked: For Good', 'Kisah akhir Glinda dan Elphaba di Oz dalam pertarungan menentukan sisi mana yang benar.', '/src/assets/film-5.webp', 8.2, 2025),
('Now You See Me', 'Para pesulap jenius menjalankan ilusi besar untuk membongkar konspirasi internasional.', '/src/assets/film-6.webp', 7.9, 2013),
('The Running Man', 'Seorang pria dipaksa mengikuti permainan mematikan: bertahan 30 hari sementara dunia memburunya.', '/src/assets/film-7.webp', 8.0, 2025),
('Keeper', 'Seorang perempuan tersadar bahwa pria yang bersamanya menyimpan obsesi berbahaya yang mengancam hidupnya.', '/src/assets/film-8.webp', 7.4, 2025);
