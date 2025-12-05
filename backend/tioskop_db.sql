-- phpMyAdmin SQL Dump
-- version 5.2.0
-- https://www.phpmyadmin.net/
--
-- Host: localhost:3306
-- Generation Time: Nov 29, 2025 at 02:00 PM
-- Server version: 8.0.30
-- PHP Version: 8.1.10

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";

START TRANSACTION;

SET time_zone = "+00:00";

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */
;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */
;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */
;
/*!40101 SET NAMES utf8mb4 */
;

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
    `total_price` decimal(10, 2) DEFAULT NULL,
    `payment_status` enum(
        'PENDING',
        'PAID',
        'CANCELLED'
    ) DEFAULT 'PENDING',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `bookings`
--

INSERT INTO
    `bookings` (
        `id`,
        `user_id`,
        `showtime_id`,
        `booking_code`,
        `total_price`,
        `payment_status`,
        `created_at`
    )
VALUES (
        4,
        2,
        1,
        'BK1764154120',
        '55000.00',
        'PENDING',
        '2025-11-26 10:48:40'
    ),
    (
        8,
        2,
        4,
        'BK1764250152',
        '70000.00',
        'PENDING',
        '2025-11-27 13:29:12'
    ),
    (
        9,
        2,
        4,
        'BK1764250163',
        '140000.00',
        'PAID',
        '2025-11-27 13:29:23'
    ),
    (
        10,
        2,
        5,
        'BK1764251441',
        '180000.00',
        'PENDING',
        '2025-11-27 13:50:41'
    ),
    (
        11,
        2,
        1,
        'BK1764251643',
        '110000.00',
        'PENDING',
        '2025-11-27 13:54:03'
    ),
    (
        12,
        2,
        1,
        'BK1764251672',
        '110000.00',
        'PAID',
        '2025-11-27 13:54:32'
    ),
    (
        13,
        2,
        1,
        'BK1764251716',
        '55000.00',
        'PAID',
        '2025-11-27 13:55:16'
    ),
    (
        14,
        1,
        4,
        'BK1764417121',
        '70000.00',
        'PENDING',
        '2025-11-29 11:52:01'
    );

-- --------------------------------------------------------

--
-- Table structure for table `booking_seats`
--

CREATE TABLE `booking_seats` (
    `id` bigint NOT NULL,
    `booking_id` bigint DEFAULT NULL,
    `seat_id` bigint DEFAULT NULL,
    `price` decimal(10, 2) DEFAULT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `booking_seats`
--

INSERT INTO
    `booking_seats` (
        `id`,
        `booking_id`,
        `seat_id`,
        `price`
    )
VALUES (1, 4, 1, '55000.00'),
    (2, 8, 15, '70000.00'),
    (3, 9, 16, '70000.00'),
    (4, 9, 17, '70000.00'),
    (5, 10, 16, '90000.00'),
    (6, 10, 17, '90000.00'),
    (7, 11, 16, '55000.00'),
    (8, 11, 17, '55000.00'),
    (9, 12, 3, '55000.00'),
    (10, 12, 4, '55000.00'),
    (11, 13, 5, '55000.00'),
    (12, 14, 51, '70000.00');

-- --------------------------------------------------------

--
-- Table structure for table `cinemas`
--

CREATE TABLE `cinemas` (
    `id` bigint NOT NULL,
    `name` varchar(100) NOT NULL,
    `address` text,
    `city` varchar(100) DEFAULT NULL,
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `user_id` bigint DEFAULT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `cinemas`
--

INSERT INTO
    `cinemas` (
        `id`,
        `name`,
        `address`,
        `city`,
        `created_at`,
        `user_id`
    )
VALUES (
        1,
        'XXI Studio BOS Mall',
        'BOS Mall',
        'Balikpapan',
        '2025-11-26 10:37:05',
        1
    ),
    (
        2,
        'CGV Plaza Balikpapan',
        'Plaza Balikpapan',
        'Balikpapan',
        '2025-11-26 10:37:05',
        1
    ),
    (
        3,
        'Cinepolis Living Pla',
        'Living Plaza',
        'Balikpapan',
        '2025-11-26 10:37:05',
        1
    );

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
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `movies`
--

INSERT INTO
    `movies` (
        `id`,
        `title`,
        `genre`,
        `rating`,
        `duration`,
        `description`,
        `poster_url`,
        `release_date`,
        `created_at`
    )
VALUES (
        1,
        'Deadpool & Wolverine',
        'Action, Comedy',
        '8.4',
        130,
        'Kolaborasi anti-hero Marvel penuh humor brutal',
        '/film-1.webp',
        '2024-09-21',
        '2025-11-26 10:38:17'
    ),
    (
        2,
        'Inside Out 2',
        'Animation, Family',
        '8.9',
        100,
        'Petualangan emosional baru dalam diri Riley',
        '/film-2.webp',
        '2024-06-14',
        '2025-11-26 10:38:17'
    ),
    (
        3,
        'Avatar: The Way of Water',
        'Sci-Fi',
        '10',
        192,
        'Kembali ke Pandora dengan petualangan air',
        '/film-3.webp',
        '2023-12-16',
        '2025-11-26 10:38:17'
    ),
    (
        4,
        'Agak Laen 2',
        'Komedi',
        '8.5',
        120,
        'Komedi lucu aja',
        '/film-5.webp',
        '2025-11-27',
        '2025-11-26 10:38:17'
    );

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
    `seat_status` enum('AVAILABLE', 'BROKEN') DEFAULT 'AVAILABLE'
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `seats`
--

INSERT INTO
    `seats` (
        `id`,
        `studio_id`,
        `seat_code`,
        `seat_row`,
        `seat_col`,
        `seat_status`
    )
VALUES (1, 1, 'A1', 1, 1, 'AVAILABLE'),
    (2, 1, 'A2', 1, 2, 'AVAILABLE'),
    (3, 1, 'A3', 1, 3, 'AVAILABLE'),
    (4, 1, 'A4', 1, 4, 'AVAILABLE'),
    (5, 1, 'A5', 1, 5, 'AVAILABLE'),
    (6, 1, 'A6', 1, 6, 'AVAILABLE'),
    (7, 1, 'A7', 1, 7, 'AVAILABLE'),
    (8, 1, 'A8', 1, 8, 'AVAILABLE'),
    (9, 1, 'A9', 1, 9, 'AVAILABLE'),
    (
        10,
        1,
        'A10',
        1,
        10,
        'AVAILABLE'
    ),
    (
        11,
        1,
        'B1',
        2,
        1,
        'AVAILABLE'
    ),
    (
        12,
        1,
        'B2',
        2,
        2,
        'AVAILABLE'
    ),
    (
        13,
        1,
        'B3',
        2,
        3,
        'AVAILABLE'
    ),
    (
        14,
        1,
        'B4',
        2,
        4,
        'AVAILABLE'
    ),
    (
        15,
        1,
        'B5',
        2,
        5,
        'AVAILABLE'
    ),
    (
        16,
        1,
        'B6',
        2,
        6,
        'AVAILABLE'
    ),
    (
        17,
        1,
        'B7',
        2,
        7,
        'AVAILABLE'
    ),
    (
        18,
        1,
        'B8',
        2,
        8,
        'AVAILABLE'
    ),
    (
        19,
        1,
        'B9',
        2,
        9,
        'AVAILABLE'
    ),
    (
        20,
        1,
        'B10',
        2,
        10,
        'AVAILABLE'
    ),
    (
        21,
        1,
        'C1',
        3,
        1,
        'AVAILABLE'
    ),
    (
        22,
        1,
        'C2',
        3,
        2,
        'AVAILABLE'
    ),
    (
        23,
        1,
        'C3',
        3,
        3,
        'AVAILABLE'
    ),
    (
        24,
        1,
        'C4',
        3,
        4,
        'AVAILABLE'
    ),
    (
        25,
        1,
        'C5',
        3,
        5,
        'AVAILABLE'
    ),
    (
        26,
        1,
        'C6',
        3,
        6,
        'AVAILABLE'
    ),
    (
        27,
        1,
        'C7',
        3,
        7,
        'AVAILABLE'
    ),
    (
        28,
        1,
        'C8',
        3,
        8,
        'AVAILABLE'
    ),
    (
        29,
        1,
        'C9',
        3,
        9,
        'AVAILABLE'
    ),
    (
        30,
        1,
        'C10',
        3,
        10,
        'AVAILABLE'
    ),
    (
        31,
        1,
        'D1',
        4,
        1,
        'AVAILABLE'
    ),
    (
        32,
        1,
        'D2',
        4,
        2,
        'AVAILABLE'
    ),
    (
        33,
        1,
        'D3',
        4,
        3,
        'AVAILABLE'
    ),
    (
        34,
        1,
        'D4',
        4,
        4,
        'AVAILABLE'
    ),
    (
        35,
        1,
        'D5',
        4,
        5,
        'AVAILABLE'
    ),
    (
        36,
        1,
        'D6',
        4,
        6,
        'AVAILABLE'
    ),
    (
        37,
        1,
        'D7',
        4,
        7,
        'AVAILABLE'
    ),
    (
        38,
        1,
        'D8',
        4,
        8,
        'AVAILABLE'
    ),
    (
        39,
        1,
        'D9',
        4,
        9,
        'AVAILABLE'
    ),
    (
        40,
        1,
        'D10',
        4,
        10,
        'AVAILABLE'
    ),
    (
        41,
        1,
        'E1',
        5,
        1,
        'AVAILABLE'
    ),
    (
        42,
        1,
        'E2',
        5,
        2,
        'AVAILABLE'
    ),
    (
        43,
        1,
        'E3',
        5,
        3,
        'AVAILABLE'
    ),
    (
        44,
        1,
        'E4',
        5,
        4,
        'AVAILABLE'
    ),
    (
        45,
        1,
        'E5',
        5,
        5,
        'AVAILABLE'
    ),
    (
        46,
        1,
        'E6',
        5,
        6,
        'AVAILABLE'
    ),
    (
        47,
        1,
        'E7',
        5,
        7,
        'AVAILABLE'
    ),
    (
        48,
        1,
        'E8',
        5,
        8,
        'AVAILABLE'
    ),
    (
        49,
        1,
        'E9',
        5,
        9,
        'AVAILABLE'
    ),
    (
        50,
        1,
        'E10',
        5,
        10,
        'AVAILABLE'
    ),
    (
        51,
        2,
        'A1',
        1,
        1,
        'AVAILABLE'
    ),
    (
        52,
        2,
        'A2',
        1,
        2,
        'AVAILABLE'
    ),
    (
        53,
        2,
        'A3',
        1,
        3,
        'AVAILABLE'
    ),
    (
        54,
        2,
        'A4',
        1,
        4,
        'AVAILABLE'
    ),
    (
        55,
        2,
        'A5',
        1,
        5,
        'AVAILABLE'
    ),
    (
        56,
        2,
        'A6',
        1,
        6,
        'AVAILABLE'
    ),
    (
        57,
        2,
        'A7',
        1,
        7,
        'AVAILABLE'
    ),
    (
        58,
        2,
        'A8',
        1,
        8,
        'AVAILABLE'
    ),
    (
        59,
        2,
        'A9',
        1,
        9,
        'AVAILABLE'
    ),
    (
        60,
        2,
        'A10',
        1,
        10,
        'AVAILABLE'
    ),
    (
        61,
        2,
        'B1',
        2,
        1,
        'AVAILABLE'
    ),
    (
        62,
        2,
        'B2',
        2,
        2,
        'AVAILABLE'
    ),
    (
        63,
        2,
        'B3',
        2,
        3,
        'AVAILABLE'
    ),
    (
        64,
        2,
        'B4',
        2,
        4,
        'AVAILABLE'
    ),
    (
        65,
        2,
        'B5',
        2,
        5,
        'AVAILABLE'
    ),
    (
        66,
        2,
        'B6',
        2,
        6,
        'AVAILABLE'
    ),
    (
        67,
        2,
        'B7',
        2,
        7,
        'AVAILABLE'
    ),
    (
        68,
        2,
        'B8',
        2,
        8,
        'AVAILABLE'
    ),
    (
        69,
        2,
        'B9',
        2,
        9,
        'AVAILABLE'
    ),
    (
        70,
        2,
        'B10',
        2,
        10,
        'AVAILABLE'
    ),
    (
        71,
        2,
        'C1',
        3,
        1,
        'AVAILABLE'
    ),
    (
        72,
        2,
        'C2',
        3,
        2,
        'AVAILABLE'
    ),
    (
        73,
        2,
        'C3',
        3,
        3,
        'AVAILABLE'
    ),
    (
        74,
        2,
        'C4',
        3,
        4,
        'AVAILABLE'
    ),
    (
        75,
        2,
        'C5',
        3,
        5,
        'AVAILABLE'
    ),
    (
        76,
        2,
        'C6',
        3,
        6,
        'AVAILABLE'
    ),
    (
        77,
        2,
        'C7',
        3,
        7,
        'AVAILABLE'
    ),
    (
        78,
        2,
        'C8',
        3,
        8,
        'AVAILABLE'
    ),
    (
        79,
        2,
        'C9',
        3,
        9,
        'AVAILABLE'
    ),
    (
        80,
        2,
        'C10',
        3,
        10,
        'AVAILABLE'
    ),
    (
        81,
        2,
        'D1',
        4,
        1,
        'AVAILABLE'
    ),
    (
        82,
        2,
        'D2',
        4,
        2,
        'AVAILABLE'
    ),
    (
        83,
        2,
        'D3',
        4,
        3,
        'AVAILABLE'
    ),
    (
        84,
        2,
        'D4',
        4,
        4,
        'AVAILABLE'
    ),
    (
        85,
        2,
        'D5',
        4,
        5,
        'AVAILABLE'
    ),
    (
        86,
        2,
        'D6',
        4,
        6,
        'AVAILABLE'
    ),
    (
        87,
        2,
        'D7',
        4,
        7,
        'AVAILABLE'
    ),
    (
        88,
        2,
        'D8',
        4,
        8,
        'AVAILABLE'
    ),
    (
        89,
        2,
        'D9',
        4,
        9,
        'AVAILABLE'
    ),
    (
        90,
        2,
        'D10',
        4,
        10,
        'AVAILABLE'
    ),
    (
        91,
        2,
        'E1',
        5,
        1,
        'AVAILABLE'
    ),
    (
        92,
        2,
        'E2',
        5,
        2,
        'AVAILABLE'
    ),
    (
        93,
        2,
        'E3',
        5,
        3,
        'AVAILABLE'
    ),
    (
        94,
        2,
        'E4',
        5,
        4,
        'AVAILABLE'
    ),
    (
        95,
        2,
        'E5',
        5,
        5,
        'AVAILABLE'
    ),
    (
        96,
        2,
        'E6',
        5,
        6,
        'AVAILABLE'
    ),
    (
        97,
        2,
        'E7',
        5,
        7,
        'AVAILABLE'
    ),
    (
        98,
        2,
        'E8',
        5,
        8,
        'AVAILABLE'
    ),
    (
        99,
        2,
        'E9',
        5,
        9,
        'AVAILABLE'
    ),
    (
        100,
        2,
        'E10',
        5,
        10,
        'AVAILABLE'
    ),
    (
        101,
        3,
        'A1',
        1,
        1,
        'AVAILABLE'
    ),
    (
        102,
        3,
        'A2',
        1,
        2,
        'AVAILABLE'
    ),
    (
        103,
        3,
        'A3',
        1,
        3,
        'AVAILABLE'
    ),
    (
        104,
        3,
        'A4',
        1,
        4,
        'AVAILABLE'
    ),
    (
        105,
        3,
        'A5',
        1,
        5,
        'AVAILABLE'
    ),
    (
        106,
        3,
        'A6',
        1,
        6,
        'AVAILABLE'
    ),
    (
        107,
        3,
        'A7',
        1,
        7,
        'AVAILABLE'
    ),
    (
        108,
        3,
        'A8',
        1,
        8,
        'AVAILABLE'
    ),
    (
        109,
        3,
        'A9',
        1,
        9,
        'AVAILABLE'
    ),
    (
        110,
        3,
        'A10',
        1,
        10,
        'AVAILABLE'
    ),
    (
        111,
        3,
        'B1',
        2,
        1,
        'AVAILABLE'
    ),
    (
        112,
        3,
        'B2',
        2,
        2,
        'AVAILABLE'
    ),
    (
        113,
        3,
        'B3',
        2,
        3,
        'AVAILABLE'
    ),
    (
        114,
        3,
        'B4',
        2,
        4,
        'AVAILABLE'
    ),
    (
        115,
        3,
        'B5',
        2,
        5,
        'AVAILABLE'
    ),
    (
        116,
        3,
        'B6',
        2,
        6,
        'AVAILABLE'
    ),
    (
        117,
        3,
        'B7',
        2,
        7,
        'AVAILABLE'
    ),
    (
        118,
        3,
        'B8',
        2,
        8,
        'AVAILABLE'
    ),
    (
        119,
        3,
        'B9',
        2,
        9,
        'AVAILABLE'
    ),
    (
        120,
        3,
        'B10',
        2,
        10,
        'AVAILABLE'
    ),
    (
        121,
        3,
        'C1',
        3,
        1,
        'AVAILABLE'
    ),
    (
        122,
        3,
        'C2',
        3,
        2,
        'AVAILABLE'
    ),
    (
        123,
        3,
        'C3',
        3,
        3,
        'AVAILABLE'
    ),
    (
        124,
        3,
        'C4',
        3,
        4,
        'AVAILABLE'
    ),
    (
        125,
        3,
        'C5',
        3,
        5,
        'AVAILABLE'
    ),
    (
        126,
        3,
        'C6',
        3,
        6,
        'AVAILABLE'
    ),
    (
        127,
        3,
        'C7',
        3,
        7,
        'AVAILABLE'
    ),
    (
        128,
        3,
        'C8',
        3,
        8,
        'AVAILABLE'
    ),
    (
        129,
        3,
        'C9',
        3,
        9,
        'AVAILABLE'
    ),
    (
        130,
        3,
        'C10',
        3,
        10,
        'AVAILABLE'
    ),
    (
        131,
        3,
        'D1',
        4,
        1,
        'AVAILABLE'
    ),
    (
        132,
        3,
        'D2',
        4,
        2,
        'AVAILABLE'
    ),
    (
        133,
        3,
        'D3',
        4,
        3,
        'AVAILABLE'
    ),
    (
        134,
        3,
        'D4',
        4,
        4,
        'AVAILABLE'
    ),
    (
        135,
        3,
        'D5',
        4,
        5,
        'AVAILABLE'
    ),
    (
        136,
        3,
        'D6',
        4,
        6,
        'AVAILABLE'
    ),
    (
        137,
        3,
        'D7',
        4,
        7,
        'AVAILABLE'
    ),
    (
        138,
        3,
        'D8',
        4,
        8,
        'AVAILABLE'
    ),
    (
        139,
        3,
        'D9',
        4,
        9,
        'AVAILABLE'
    ),
    (
        140,
        3,
        'D10',
        4,
        10,
        'AVAILABLE'
    ),
    (
        141,
        3,
        'E1',
        5,
        1,
        'AVAILABLE'
    ),
    (
        142,
        3,
        'E2',
        5,
        2,
        'AVAILABLE'
    ),
    (
        143,
        3,
        'E3',
        5,
        3,
        'AVAILABLE'
    ),
    (
        144,
        3,
        'E4',
        5,
        4,
        'AVAILABLE'
    ),
    (
        145,
        3,
        'E5',
        5,
        5,
        'AVAILABLE'
    ),
    (
        146,
        3,
        'E6',
        5,
        6,
        'AVAILABLE'
    ),
    (
        147,
        3,
        'E7',
        5,
        7,
        'AVAILABLE'
    ),
    (
        148,
        3,
        'E8',
        5,
        8,
        'AVAILABLE'
    ),
    (
        149,
        3,
        'E9',
        5,
        9,
        'AVAILABLE'
    ),
    (
        150,
        3,
        'E10',
        5,
        10,
        'AVAILABLE'
    );

-- --------------------------------------------------------

--
-- Table structure for table `showtimes`
--

CREATE TABLE `showtimes` (
    `id` bigint NOT NULL,
    `movie_id` bigint DEFAULT NULL,
    `studio_id` bigint DEFAULT NULL,
    `start_time` datetime DEFAULT NULL,
    `price` decimal(10, 2) DEFAULT NULL
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `showtimes`
--

INSERT INTO
    `showtimes` (
        `id`,
        `movie_id`,
        `studio_id`,
        `start_time`,
        `price`
    )
VALUES (
        1,
        1,
        1,
        '2025-11-27 14:00:00',
        '55000.00'
    ),
    (
        2,
        1,
        2,
        '2025-11-27 19:00:00',
        '50000.00'
    ),
    (
        3,
        2,
        3,
        '2025-11-27 13:30:00',
        '70000.00'
    ),
    (
        4,
        4,
        2,
        '2025-11-27 10:00:00',
        '70000.00'
    ),
    (
        5,
        4,
        4,
        '2025-11-29 22:00:00',
        '90000.00'
    ),
    (
        8,
        4,
        1,
        '2025-11-30 10:00:00',
        '80000.00'
    );

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
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `studios`
--

INSERT INTO
    `studios` (
        `id`,
        `cinema_id`,
        `name`,
        `capacity`,
        `type`
    )
VALUES (
        1,
        1,
        'Studio 1',
        120,
        'Dolby Atmos'
    ),
    (
        2,
        1,
        'Studio 2',
        80,
        'Regular'
    ),
    (
        3,
        2,
        'IMAX Hall',
        150,
        'IMAX'
    ),
    (
        4,
        3,
        'Studio Premiere',
        60,
        'Premier'
    );

-- --------------------------------------------------------

--
-- Table structure for table `users`
--

CREATE TABLE `users` (
    `id` bigint NOT NULL,
    `name` varchar(100) DEFAULT NULL,
    `email` varchar(100) DEFAULT NULL,
    `password` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
    `role` enum('admin', 'customer') DEFAULT 'customer',
    `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
    `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE = InnoDB DEFAULT CHARSET = utf8mb4 COLLATE = utf8mb4_general_ci;

--
-- Dumping data for table `users`
--

INSERT INTO
    `users` (
        `id`,
        `name`,
        `email`,
        `password`,
        `role`,
        `created_at`,
        `updated_at`
    )
VALUES (
        1,
        'admin',
        'admin@tioskop.com',
        'akuadmin123',
        'admin',
        '2025-11-26 10:44:27',
        '2025-11-29 13:49:46'
    ),
    (
        2,
        'Adit',
        'adit@gmail.com',
        'akuadit',
        'customer',
        '2025-11-26 10:46:38',
        '2025-11-29 13:49:46'
    ),
    (
        9,
        'John Doe',
        'john@example.com',
        'ec772c7a78ed4548',
        'customer',
        '2025-11-29 13:56:47',
        '2025-11-29 13:56:47'
    );

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
ADD PRIMARY KEY (`id`),
ADD KEY `fk_cinema_user` (`user_id`);

--
-- Indexes for table `movies`
--
ALTER TABLE `movies`
ADD PRIMARY KEY (`id`),
ADD KEY `idx_title` (`title`);

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
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 15;

--
-- AUTO_INCREMENT for table `booking_seats`
--
ALTER TABLE `booking_seats`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 13;

--
-- AUTO_INCREMENT for table `cinemas`
--
ALTER TABLE `cinemas`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 4;

--
-- AUTO_INCREMENT for table `movies`
--
ALTER TABLE `movies`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 6;

--
-- AUTO_INCREMENT for table `seats`
--
ALTER TABLE `seats`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 151;

--
-- AUTO_INCREMENT for table `showtimes`
--
ALTER TABLE `showtimes`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 9;

--
-- AUTO_INCREMENT for table `studios`
--
ALTER TABLE `studios`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 5;

--
-- AUTO_INCREMENT for table `users`
--
ALTER TABLE `users`
MODIFY `id` bigint NOT NULL AUTO_INCREMENT,
AUTO_INCREMENT = 10;

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
-- Constraints for table `cinemas`
--
ALTER TABLE `cinemas`
ADD CONSTRAINT `fk_cinema_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL ON UPDATE CASCADE;

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

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */
;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */
;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */
;