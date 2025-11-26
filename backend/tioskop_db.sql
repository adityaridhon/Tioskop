-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: localhost:3306
-- Generation Time: Nov 26, 2025 at 10:56 AM
-- Server version: 8.0.30
-- PHP Version: 8.1.10

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `tioskop_db`
--

-- --------------------------------------------------------

--
-- Table structure for table `bookings`
--

CREATE TABLE `bookings` (
  `id` bigint NOT NULL,
  `user_id` bigint DEFAULT NULL,
  `showtime_id` bigint DEFAULT NULL,
  `booking_code` varchar(20) DEFAULT NULL,
  `total_price` decimal(10,2) DEFAULT NULL,
  `payment_status` enum('PENDING','PAID','CANCELLED') DEFAULT 'PENDING',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `bookings`
--

INSERT INTO `bookings` (`id`, `user_id`, `showtime_id`, `booking_code`, `total_price`, `payment_status`, `created_at`) VALUES
(4, 2, 1, 'BK1764154120', '55000.00', 'PENDING', '2025-11-26 10:48:40');

-- --------------------------------------------------------

--
-- Table structure for table `booking_seats`
--

CREATE TABLE `booking_seats` (
  `id` bigint NOT NULL,
  `booking_id` bigint DEFAULT NULL,
  `seat_id` bigint DEFAULT NULL,
  `price` decimal(10,2) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `booking_seats`
--

INSERT INTO `booking_seats` (`id`, `booking_id`, `seat_id`, `price`) VALUES
(1, 4, 1, '55000.00');

-- --------------------------------------------------------

--
-- Table structure for table `cinemas`
--

CREATE TABLE `cinemas` (
  `id` bigint NOT NULL,
  `name` varchar(100) NOT NULL,
  `address` text,
  `city` varchar(100) DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `cinemas`
--

INSERT INTO `cinemas` (`id`, `name`, `address`, `city`, `created_at`) VALUES
(1, 'XXI Studio BOS Mall', 'BOS Mall', 'Balikpapan', '2025-11-26 10:37:05'),
(2, 'CGV Plaza Balikpapan', 'Plaza Balikpapan', 'Balikpapan', '2025-11-26 10:37:05'),
(3, 'Cinepolis Living Pla', 'Living Plaza', 'Balikpapan', '2025-11-26 10:37:05');

-- --------------------------------------------------------

--
-- Table structure for table `movies`
--

CREATE TABLE `movies` (
  `id` bigint NOT NULL,
  `title` varchar(200) NOT NULL,
  `genre` varchar(200) DEFAULT NULL,
  `rating` varchar(10) DEFAULT NULL,
  `duration` int DEFAULT NULL,
  `description` text,
  `poster_url` varchar(255) DEFAULT NULL,
  `release_date` date DEFAULT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `movies`
--

INSERT INTO `movies` (`id`, `title`, `genre`, `rating`, `duration`, `description`, `poster_url`, `release_date`, `created_at`) VALUES
(1, 'Deadpool & Wolverine', 'Action, Comedy', '8.4', 130, 'Kolaborasi anti-hero Marvel penuh humor brutal', 'poster1.jpg', '2024-09-21', '2025-11-26 10:38:17'),
(2, 'Inside Out 2', 'Animation, Family', '8.9', 100, 'Petualangan emosional baru dalam diri Riley', 'poster2.jpg', '2024-06-14', '2025-11-26 10:38:17'),
(3, 'Avatar: The Way of Water', 'Sci-Fi', '7.8', 192, 'Kembali ke Pandora dengan petualangan air', 'poster3.jpg', '2023-12-16', '2025-11-26 10:38:17'),
(4, 'Conjuring 4', 'Horror', '7.5', 140, 'Kasus gelap paling menyeramkan keluarga Warren', 'poster4.jpg', '2025-01-10', '2025-11-26 10:38:17');

-- --------------------------------------------------------

--
-- Table structure for table `seats`
--

CREATE TABLE `seats` (
  `id` bigint NOT NULL,
  `studio_id` bigint NOT NULL,
  `seat_code` varchar(10) NOT NULL,
  `seat_row` int DEFAULT NULL,
  `seat_col` int DEFAULT NULL,
  `seat_status` enum('AVAILABLE','BROKEN') DEFAULT 'AVAILABLE'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `seats`
--

INSERT INTO `seats` (`id`, `studio_id`, `seat_code`, `seat_row`, `seat_col`, `seat_status`) VALUES
(1, 1, 'A1', 1, 1, 'AVAILABLE'),
(2, 1, 'A2', 1, 2, 'AVAILABLE'),
(3, 1, 'A3', 1, 3, 'AVAILABLE'),
(4, 1, 'A4', 1, 4, 'AVAILABLE'),
(5, 1, 'A5', 1, 5, 'AVAILABLE'),
(6, 1, 'A6', 1, 6, 'AVAILABLE'),
(7, 1, 'A7', 1, 7, 'AVAILABLE'),
(8, 1, 'A8', 1, 8, 'AVAILABLE'),
(9, 1, 'A9', 1, 9, 'AVAILABLE'),
(10, 1, 'A10', 1, 10, 'AVAILABLE'),
(11, 1, 'B1', 2, 1, 'AVAILABLE'),
(12, 1, 'B2', 2, 2, 'AVAILABLE'),
(13, 1, 'B3', 2, 3, 'AVAILABLE'),
(14, 1, 'B4', 2, 4, 'AVAILABLE'),
(15, 1, 'B5', 2, 5, 'AVAILABLE'),
(16, 1, 'B6', 2, 6, 'AVAILABLE'),
(17, 1, 'B7', 2, 7, 'AVAILABLE'),
(18, 1, 'B8', 2, 8, 'AVAILABLE'),
(19, 1, 'B9', 2, 9, 'AVAILABLE'),
(20, 1, 'B10', 2, 10, 'AVAILABLE'),
(21, 1, 'C1', 3, 1, 'AVAILABLE'),
(22, 1, 'C2', 3, 2, 'AVAILABLE'),
(23, 1, 'C3', 3, 3, 'AVAILABLE'),
(24, 1, 'C4', 3, 4, 'AVAILABLE'),
(25, 1, 'C5', 3, 5, 'AVAILABLE'),
(26, 1, 'C6', 3, 6, 'AVAILABLE'),
(27, 1, 'C7', 3, 7, 'AVAILABLE'),
(28, 1, 'C8', 3, 8, 'AVAILABLE'),
(29, 1, 'C9', 3, 9, 'AVAILABLE'),
(30, 1, 'C10', 3, 10, 'AVAILABLE'),
(31, 1, 'D1', 4, 1, 'AVAILABLE'),
(32, 1, 'D2', 4, 2, 'AVAILABLE'),
(33, 1, 'D3', 4, 3, 'AVAILABLE'),
(34, 1, 'D4', 4, 4, 'AVAILABLE'),
(35, 1, 'D5', 4, 5, 'AVAILABLE'),
(36, 1, 'D6', 4, 6, 'AVAILABLE'),
(37, 1, 'D7', 4, 7, 'AVAILABLE'),
(38, 1, 'D8', 4, 8, 'AVAILABLE'),
(39, 1, 'D9', 4, 9, 'AVAILABLE'),
(40, 1, 'D10', 4, 10, 'AVAILABLE'),
(41, 1, 'E1', 5, 1, 'AVAILABLE'),
(42, 1, 'E2', 5, 2, 'AVAILABLE'),
(43, 1, 'E3', 5, 3, 'AVAILABLE'),
(44, 1, 'E4', 5, 4, 'AVAILABLE'),
(45, 1, 'E5', 5, 5, 'AVAILABLE'),
(46, 1, 'E6', 5, 6, 'AVAILABLE'),
(47, 1, 'E7', 5, 7, 'AVAILABLE'),
(48, 1, 'E8', 5, 8, 'AVAILABLE'),
(49, 1, 'E9', 5, 9, 'AVAILABLE'),
(50, 1, 'E10', 5, 10, 'AVAILABLE');

-- --------------------------------------------------------

--
-- Table structure for table `showtimes`
--

CREATE TABLE `showtimes` (
  `id` bigint NOT NULL,
  `movie_id` bigint DEFAULT NULL,
  `studio_id` bigint DEFAULT NULL,
  `start_time` datetime DEFAULT NULL,
  `price` decimal(10,2) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `showtimes`
--

INSERT INTO `showtimes` (`id`, `movie_id`, `studio_id`, `start_time`, `price`) VALUES
(1, 1, 1, '2025-11-27 14:00:00', '55000.00'),
(2, 1, 2, '2025-11-27 19:00:00', '50000.00'),
(3, 2, 3, '2025-11-27 13:30:00', '70000.00'),
(4, 3, 1, '2025-11-28 20:00:00', '80000.00'),
(5, 4, 4, '2025-11-29 22:00:00', '90000.00');

-- --------------------------------------------------------

--
-- Table structure for table `studios`
--

CREATE TABLE `studios` (
  `id` bigint NOT NULL,
  `cinema_id` bigint DEFAULT NULL,
  `name` varchar(50) NOT NULL,
  `capacity` int NOT NULL,
  `type` varchar(50) DEFAULT 'REGULAR'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `studios`
--

INSERT INTO `studios` (`id`, `cinema_id`, `name`, `capacity`, `type`) VALUES
(1, 1, 'Studio 1', 120, 'Dolby Atmos'),
(2, 1, 'Studio 2', 80, 'Regular'),
(3, 2, 'IMAX Hall', 150, 'IMAX'),
(4, 3, 'Studio Premiere', 60, 'Premier');

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

CREATE TABLE `users` (
  `id` bigint NOT NULL,
  `name` varchar(100) DEFAULT NULL,
  `email` varchar(100) DEFAULT NULL,
  `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL,
  `role` enum('admin','customer') DEFAULT 'customer',
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `users`
--

INSERT INTO `users` (`id`, `name`, `email`, `password`, `role`, `created_at`) VALUES
(1, 'admin', 'admin@tioskop.com', 'akuadmin123', 'admin', '2025-11-26 10:44:27'),
(2, 'Adit', 'adit@gmail.com', 'akuadit', 'customer', '2025-11-26 10:46:38');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `bookings`
--
ALTER TABLE `bookings`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `booking_code` (`booking_code`),
  ADD KEY `user_id` (`user_id`),
  ADD KEY `showtime_id` (`showtime_id`);

--
-- Indexes for table `booking_seats`
--
ALTER TABLE `booking_seats`
  ADD PRIMARY KEY (`id`),
  ADD KEY `booking_id` (`booking_id`),
  ADD KEY `seat_id` (`seat_id`);

--
-- Indexes for table `cinemas`
--
ALTER TABLE `cinemas`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `movies`
--
ALTER TABLE `movies`
  ADD PRIMARY KEY (`id`);

--
-- Indexes for table `seats`
--
ALTER TABLE `seats`
  ADD PRIMARY KEY (`id`),
  ADD KEY `fk_seats_studio` (`studio_id`);

--
-- Indexes for table `showtimes`
--
ALTER TABLE `showtimes`
  ADD PRIMARY KEY (`id`),
  ADD KEY `movie_id` (`movie_id`),
  ADD KEY `studio_id` (`studio_id`);

--
-- Indexes for table `studios`
--
ALTER TABLE `studios`
  ADD PRIMARY KEY (`id`),
  ADD KEY `cinema_id` (`cinema_id`);

--
-- Indexes for table `users`
--
ALTER TABLE `users`
  ADD PRIMARY KEY (`id`),
  ADD UNIQUE KEY `email` (`email`);

--
-- AUTO_INCREMENT for dumped tables
--

--
-- AUTO_INCREMENT for table `bookings`
--
ALTER TABLE `bookings`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=8;

--
-- AUTO_INCREMENT for table `booking_seats`
--
ALTER TABLE `booking_seats`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=2;

--
-- AUTO_INCREMENT for table `cinemas`
--
ALTER TABLE `cinemas`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=4;

--
-- AUTO_INCREMENT for table `movies`
--
ALTER TABLE `movies`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=5;

--
-- AUTO_INCREMENT for table `seats`
--
ALTER TABLE `seats`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=51;

--
-- AUTO_INCREMENT for table `showtimes`
--
ALTER TABLE `showtimes`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=6;

--
-- AUTO_INCREMENT for table `studios`
--
ALTER TABLE `studios`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=5;

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
  MODIFY `id` bigint NOT NULL AUTO_INCREMENT, AUTO_INCREMENT=3;

--
-- Constraints for dumped tables
--

--
-- Constraints for table `bookings`
--
ALTER TABLE `bookings`
  ADD CONSTRAINT `bookings_ibfk_1` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`),
  ADD CONSTRAINT `bookings_ibfk_2` FOREIGN KEY (`showtime_id`) REFERENCES `showtimes` (`id`);

--
-- Constraints for table `booking_seats`
--
ALTER TABLE `booking_seats`
  ADD CONSTRAINT `booking_seats_ibfk_1` FOREIGN KEY (`booking_id`) REFERENCES `bookings` (`id`) ON DELETE CASCADE,
  ADD CONSTRAINT `booking_seats_ibfk_2` FOREIGN KEY (`seat_id`) REFERENCES `seats` (`id`);

--
-- Constraints for table `seats`
--
ALTER TABLE `seats`
  ADD CONSTRAINT `fk_seats_studio` FOREIGN KEY (`studio_id`) REFERENCES `studios` (`id`) ON DELETE CASCADE;

--
-- Constraints for table `showtimes`
--
ALTER TABLE `showtimes`
  ADD CONSTRAINT `showtimes_ibfk_1` FOREIGN KEY (`movie_id`) REFERENCES `movies` (`id`),
  ADD CONSTRAINT `showtimes_ibfk_2` FOREIGN KEY (`studio_id`) REFERENCES `studios` (`id`);

--
-- Constraints for table `studios`
--
ALTER TABLE `studios`
  ADD CONSTRAINT `studios_ibfk_1` FOREIGN KEY (`cinema_id`) REFERENCES `cinemas` (`id`) ON DELETE CASCADE;
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;

/* sp_resister_user stored procedure */
DELIMITER $$
CREATE DEFINER=`root`@`localhost` PROCEDURE `sp_register_user`(
    IN p_name VARCHAR(100),
    IN p_email VARCHAR(100),
    IN p_password VARCHAR(255),
    IN p_role ENUM('admin','customer')
)
BEGIN
    -- Cek email sudah dipakai atau belum
    IF EXISTS (SELECT 1 FROM users WHERE email = p_email) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Email sudah terdaftar';
    END IF;

    INSERT INTO users (name, email, password, role)
    VALUES (p_name, p_email, p_password, p_role);
    
    SELECT LAST_INSERT_ID() AS user_id;
END$$
DELIMITER ;

/* sp_login_user stored procedure */
DELIMITER $$
CREATE DEFINER=`root`@`localhost` PROCEDURE `sp_login_user`(
    IN p_email VARCHAR(100),
    IN p_password VARCHAR(255)
)
BEGIN
    DECLARE v_user_id BIGINT;

    SELECT id
    INTO v_user_id
    FROM users
    WHERE email = p_email
      AND password = p_password
    LIMIT 1;

    IF v_user_id IS NULL THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Email atau password salah';
    END IF;

    SELECT id, name, email, role, created_at
    FROM users
    WHERE id = v_user_id;
END$$
DELIMITER ;

DELIMITER $$

/* sp_jadwal_kota_tanggal stored procedure */
CREATE PROCEDURE sp_jadwal_kota_tanggal (
    IN p_kota VARCHAR(100),
    IN p_tanggal DATE
)
BEGIN
    SELECT 
        st.id AS showtime_id,
        m.id AS movie_id,
        m.title,
        m.genre,
        m.rating,
        m.duration,
        c.name AS nama_bioskop,
        c.city,
        s.name AS studio,
        s.type AS jenis_studio,
        st.start_time,
        st.price
    FROM showtimes st
    JOIN movies m ON m.id = st.movie_id
    JOIN studios s ON s.id = st.studio_id
    JOIN cinemas c ON c.id = s.cinema_id
    WHERE c.city = p_kota
      AND DATE(st.start_time) = p_tanggal
    ORDER BY m.title, st.start_time;
END$$

DELIMITER ;


/* sp_kusi_tersedia */

DELIMITER $$

CREATE PROCEDURE sp_kursi_tersedia (
    IN p_showtime_id BIGINT
)
BEGIN
    SELECT 
        s.id,
        s.studio_id,
        s.seat_code,
        s.seat_row,
        s.seat_col,
        s.seat_status
    FROM seats s
    JOIN showtimes st ON st.studio_id = s.studio_id
    WHERE st.id = p_showtime_id
      AND s.seat_status = 'AVAILABLE'
      AND s.id NOT IN (
          SELECT bs.seat_id
          FROM booking_seats bs
          JOIN bookings b ON b.id = bs.booking_id
          WHERE b.showtime_id = p_showtime_id
      )
    ORDER BY s.seat_row, s.seat_col;
END$$

DELIMITER ;



/* sp_buat_pemesanan_satu_kursi */
DELIMITER $$

CREATE PROCEDURE sp_buat_pemesanan_satu_kursi (
    IN p_user_id BIGINT,
    IN p_showtime_id BIGINT,
    IN p_seat_id BIGINT
)
BEGIN
    DECLARE v_price DECIMAL(10,2);
    DECLARE v_booking_id BIGINT;
    DECLARE v_kode VARCHAR(20);

    SELECT price INTO v_price
    FROM showtimes WHERE id = p_showtime_id;

    IF v_price IS NULL THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Jadwal tidak ditemukan';
    END IF;

    IF NOT EXISTS (
        SELECT 1
        FROM seats s
        JOIN showtimes st ON st.studio_id = s.studio_id
        WHERE st.id = p_showtime_id AND s.id = p_seat_id
    ) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Kursi tidak valid untuk jadwal ini';
    END IF;

    IF EXISTS (
        SELECT 1
        FROM booking_seats bs
        JOIN bookings b ON b.id = bs.booking_id
        WHERE b.showtime_id = p_showtime_id
          AND bs.seat_id = p_seat_id
    ) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Kursi sudah dipesan';
    END IF;

    SET v_kode = CONCAT('BK', UNIX_TIMESTAMP());

    INSERT INTO bookings (user_id, showtime_id, booking_code, total_price, payment_status)
    VALUES (p_user_id, p_showtime_id, v_kode, v_price, 'PENDING');

    SET v_booking_id = LAST_INSERT_ID();

    INSERT INTO booking_seats (booking_id, seat_id, price)
    VALUES (v_booking_id, p_seat_id, v_price);

    SELECT v_booking_id AS booking_id, v_kode AS booking_code, v_price AS total_price;
END$$

DELIMITER ;


/* sp_tandai_pembayaran */
DELIMITER $$

CREATE PROCEDURE sp_tandai_pembayaran (
    IN p_booking_id BIGINT
)
BEGIN
    IF NOT EXISTS (SELECT 1 FROM bookings WHERE id = p_booking_id) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Pemesanan tidak ditemukan';
    END IF;

    UPDATE bookings
    SET payment_status = 'PAID'
    WHERE id = p_booking_id;

    SELECT * FROM bookings WHERE id = p_booking_id;
END$$

DELIMITER ;

/* sp_batalkan_pemesanan */
DELIMITER $$

CREATE PROCEDURE sp_batalkan_pemesanan (
    IN p_booking_id BIGINT
)
BEGIN
    IF NOT EXISTS (SELECT 1 FROM bookings WHERE id = p_booking_id) THEN
        SIGNAL SQLSTATE '45000'
            SET MESSAGE_TEXT = 'Pemesanan tidak ditemukan';
    END IF;

    UPDATE bookings
    SET payment_status = 'CANCELLED'
    WHERE id = p_booking_id;

    SELECT * FROM bookings WHERE id = p_booking_id;
END$$

DELIMITER ;
