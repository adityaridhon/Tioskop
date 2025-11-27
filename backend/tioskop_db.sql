-- Drop database if exists and create new
DROP DATABASE IF EXISTS tioskop_db;
CREATE DATABASE tioskop_db;
USE tioskop_db;

-- ============================================
-- TABLE DEFINITIONS
-- ============================================

-- Table: users
CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    full_name VARCHAR(100) NOT NULL,
    phone VARCHAR(20),
    role ENUM('ADMIN', 'USER') DEFAULT 'USER',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table: movies
CREATE TABLE movies (
    id INT AUTO_INCREMENT PRIMARY KEY,
    title VARCHAR(200) NOT NULL,
    description TEXT,
    duration INT NOT NULL, -- durasi dalam menit
    genre VARCHAR(100),
    rating VARCHAR(10), -- PG, PG-13, R, dll
    release_date DATE,
    poster_url VARCHAR(255),
    trailer_url VARCHAR(255),
    director VARCHAR(100),
    cast TEXT, -- bisa simpan sebagai JSON atau comma-separated
    status ENUM('COMING_SOON', 'NOW_SHOWING', 'ENDED') DEFAULT 'NOW_SHOWING',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table: studios
CREATE TABLE studios (
    id INT AUTO_INCREMENT PRIMARY KEY,
    studio_name VARCHAR(50) NOT NULL,
    studio_type ENUM('REGULAR', 'PREMIERE', 'IMAX', 'DOLBY') DEFAULT 'REGULAR',
    total_seats INT NOT NULL,
    total_rows INT NOT NULL,
    total_cols INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Table: seats
CREATE TABLE seats (
    id INT AUTO_INCREMENT PRIMARY KEY,
    studio_id INT NOT NULL,
    seat_code VARCHAR(10) NOT NULL, -- contoh: A1, B5, C10
    seat_row VARCHAR(5) NOT NULL, -- A, B, C, dll
    seat_col INT NOT NULL, -- 1, 2, 3, dll
    seat_status ENUM('AVAILABLE', 'MAINTENANCE') DEFAULT 'AVAILABLE',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (studio_id) REFERENCES studios(id) ON DELETE CASCADE,
    UNIQUE KEY unique_seat (studio_id, seat_code)
);

-- Table: showtimes
CREATE TABLE showtimes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    movie_id INT NOT NULL,
    studio_id INT NOT NULL,
    show_date DATE NOT NULL,
    show_time TIME NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    status ENUM('AVAILABLE', 'FULL', 'CANCELLED') DEFAULT 'AVAILABLE',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (movie_id) REFERENCES movies(id) ON DELETE CASCADE,
    FOREIGN KEY (studio_id) REFERENCES studios(id) ON DELETE CASCADE
);

-- Table: bookings
CREATE TABLE bookings (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    showtime_id INT NOT NULL,
    booking_code VARCHAR(50) NOT NULL UNIQUE,
    total_price DECIMAL(10,2) NOT NULL,
    payment_status ENUM('PENDING', 'PAID', 'CANCELLED') DEFAULT 'PENDING',
    payment_method VARCHAR(50),
    payment_date TIMESTAMP NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (showtime_id) REFERENCES showtimes(id) ON DELETE CASCADE
);

-- Table: booking_seats (junction table)
CREATE TABLE booking_seats (
    id INT AUTO_INCREMENT PRIMARY KEY,
    booking_id INT NOT NULL,
    seat_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (booking_id) REFERENCES bookings(id) ON DELETE CASCADE,
    FOREIGN KEY (seat_id) REFERENCES seats(id) ON DELETE CASCADE,
    UNIQUE KEY unique_booking_seat (booking_id, seat_id)
);

-- ============================================
-- INDEXES FOR PERFORMANCE
-- ============================================

CREATE INDEX idx_movies_status ON movies(status);
CREATE INDEX idx_movies_release_date ON movies(release_date);
CREATE INDEX idx_showtimes_date ON showtimes(show_date);
CREATE INDEX idx_showtimes_movie ON showtimes(movie_id);
CREATE INDEX idx_showtimes_studio ON showtimes(studio_id);
CREATE INDEX idx_bookings_user ON bookings(user_id);
CREATE INDEX idx_bookings_showtime ON bookings(showtime_id);
CREATE INDEX idx_bookings_status ON bookings(payment_status);
CREATE INDEX idx_booking_seats_booking ON booking_seats(booking_id);
CREATE INDEX idx_booking_seats_seat ON booking_seats(seat_id);

-- ============================================
-- STORED PROCEDURES
-- ============================================

-- Procedure 1: Get Available Seats with Booking Status
DELIMITER //
DROP PROCEDURE IF EXISTS GetAvailableSeatsWithBookingStatus//
CREATE PROCEDURE GetAvailableSeatsWithBookingStatus(IN p_showtime_id INT)
BEGIN
    SELECT 
        s.id,
        s.studio_id,
        s.seat_code,
        s.seat_row,
        s.seat_col,
        s.seat_status,
        CASE 
            WHEN bs.booking_id IS NOT NULL THEN TRUE 
            ELSE FALSE 
        END as is_booked,
        bs.booking_id,
        b.payment_status
    FROM seats s
    JOIN showtimes st ON s.studio_id = st.studio_id
    LEFT JOIN booking_seats bs ON bs.seat_id = s.id
        AND bs.booking_id IN (
            SELECT b.id FROM bookings b 
            WHERE b.showtime_id = st.id 
            AND b.payment_status != 'CANCELLED'
        )
    LEFT JOIN bookings b ON b.id = bs.booking_id
    WHERE st.id = p_showtime_id
        AND s.seat_status = 'AVAILABLE'
    ORDER BY s.seat_row, s.seat_col;
END //
DELIMITER ;

-- Procedure 2: Create Booking with Seats Transaction
DELIMITER //
DROP PROCEDURE IF EXISTS CreateBookingWithSeats//
CREATE PROCEDURE CreateBookingWithSeats(
    IN p_user_id INT,
    IN p_showtime_id INT,
    IN p_seat_ids TEXT,
    IN p_booking_code VARCHAR(50),
    OUT p_booking_id INT,
    OUT p_total_price DECIMAL(10,2),
    OUT p_message VARCHAR(255)
)
BEGIN
    DECLARE v_price DECIMAL(10,2);
    DECLARE v_seat_count INT;
    DECLARE v_seat_id INT;
    DECLARE v_pos INT;
    DECLARE v_remaining_ids TEXT;
    DECLARE v_booked_seats INT;
    
    DECLARE EXIT HANDLER FOR SQLEXCEPTION
    BEGIN
        ROLLBACK;
        SET p_message = 'Error: Transaksi booking gagal';
        SET p_booking_id = NULL;
    END;
    
    START TRANSACTION;
    
    -- Get showtime price
    SELECT price INTO v_price 
    FROM showtimes 
    WHERE id = p_showtime_id AND status = 'AVAILABLE';
    
    IF v_price IS NULL THEN
        SET p_message = 'Error: Jadwal tayang tidak tersedia';
        ROLLBACK;
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Jadwal tayang tidak tersedia';
    END IF;
    
    -- Check if any selected seats are already booked
    SET v_remaining_ids = p_seat_ids;
    SET v_booked_seats = 0;
    
    WHILE LENGTH(v_remaining_ids) > 0 DO
        SET v_pos = LOCATE(',', v_remaining_ids);
        
        IF v_pos > 0 THEN
            SET v_seat_id = CAST(SUBSTRING(v_remaining_ids, 1, v_pos - 1) AS UNSIGNED);
            SET v_remaining_ids = SUBSTRING(v_remaining_ids, v_pos + 1);
        ELSE
            SET v_seat_id = CAST(v_remaining_ids AS UNSIGNED);
            SET v_remaining_ids = '';
        END IF;
        
        -- Check if seat is already booked
        SELECT COUNT(*) INTO v_booked_seats
        FROM booking_seats bs
        JOIN bookings b ON b.id = bs.booking_id
        WHERE bs.seat_id = v_seat_id
            AND b.showtime_id = p_showtime_id
            AND b.payment_status != 'CANCELLED';
        
        IF v_booked_seats > 0 THEN
            SET p_message = CONCAT('Error: Kursi dengan ID ', v_seat_id, ' sudah dibooking');
            ROLLBACK;
            SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Kursi sudah dibooking';
        END IF;
    END WHILE;
    
    -- Count seats
    SET v_seat_count = (LENGTH(p_seat_ids) - LENGTH(REPLACE(p_seat_ids, ',', '')) + 1);
    SET p_total_price = v_price * v_seat_count;
    
    -- Insert booking
    INSERT INTO bookings (user_id, showtime_id, booking_code, total_price, payment_status)
    VALUES (p_user_id, p_showtime_id, p_booking_code, p_total_price, 'PENDING');
    
    SET p_booking_id = LAST_INSERT_ID();
    
    -- Insert booking_seats
    SET v_remaining_ids = p_seat_ids;
    
    WHILE LENGTH(v_remaining_ids) > 0 DO
        SET v_pos = LOCATE(',', v_remaining_ids);
        
        IF v_pos > 0 THEN
            SET v_seat_id = CAST(SUBSTRING(v_remaining_ids, 1, v_pos - 1) AS UNSIGNED);
            SET v_remaining_ids = SUBSTRING(v_remaining_ids, v_pos + 1);
        ELSE
            SET v_seat_id = CAST(v_remaining_ids AS UNSIGNED);
            SET v_remaining_ids = '';
        END IF;
        
        INSERT INTO booking_seats (booking_id, seat_id)
        VALUES (p_booking_id, v_seat_id);
    END WHILE;
    
    SET p_message = 'Success: Booking berhasil dibuat';
    COMMIT;
END //
DELIMITER ;

-- Procedure 3: Get Booking Details
DELIMITER //
DROP PROCEDURE IF EXISTS GetBookingDetails//
CREATE PROCEDURE GetBookingDetails(IN p_booking_code VARCHAR(50))
BEGIN
    SELECT 
        b.id as booking_id,
        b.booking_code,
        b.total_price,
        b.payment_status,
        b.payment_method,
        b.payment_date,
        b.created_at as booking_date,
        u.id as user_id,
        u.full_name,
        u.email,
        u.phone,
        m.id as movie_id,
        m.title as movie_title,
        m.duration,
        m.rating,
        m.poster_url,
        st.show_date,
        st.show_time,
        st.price as ticket_price,
        s.studio_name,
        s.studio_type,
        GROUP_CONCAT(se.seat_code ORDER BY se.seat_row, se.seat_col) as seats
    FROM bookings b
    JOIN users u ON b.user_id = u.id
    JOIN showtimes st ON b.showtime_id = st.id
    JOIN movies m ON st.movie_id = m.id
    JOIN studios s ON st.studio_id = s.id
    JOIN booking_seats bs ON b.id = bs.booking_id
    JOIN seats se ON bs.seat_id = se.id
    WHERE b.booking_code = p_booking_code
    GROUP BY b.id;
END //
DELIMITER ;

-- Procedure 4: Update Payment Status
DELIMITER //
DROP PROCEDURE IF EXISTS UpdatePaymentStatus//
CREATE PROCEDURE UpdatePaymentStatus(
    IN p_booking_code VARCHAR(50),
    IN p_payment_status VARCHAR(20),
    IN p_payment_method VARCHAR(50)
)
BEGIN
    DECLARE v_booking_id INT;
    
    SELECT id INTO v_booking_id 
    FROM bookings 
    WHERE booking_code = p_booking_code;
    
    IF v_booking_id IS NULL THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Booking tidak ditemukan';
    END IF;
    
    UPDATE bookings 
    SET 
        payment_status = p_payment_status,
        payment_method = p_payment_method,
        payment_date = IF(p_payment_status = 'PAID', NOW(), NULL)
    WHERE id = v_booking_id;
    
    SELECT 'Success' as status, 'Payment status updated' as message;
END //
DELIMITER ;

-- Procedure 5: Get Showtimes by Movie and Date
DELIMITER //
DROP PROCEDURE IF EXISTS GetShowtimesByMovieAndDate//
CREATE PROCEDURE GetShowtimesByMovieAndDate(
    IN p_movie_id INT,
    IN p_show_date DATE
)
BEGIN
    SELECT 
        st.id,
        st.show_time,
        st.price,
        st.status,
        s.studio_name,
        s.studio_type,
        s.total_seats,
        COUNT(DISTINCT bs.seat_id) as booked_seats,
        (s.total_seats - COUNT(DISTINCT bs.seat_id)) as available_seats
    FROM showtimes st
    JOIN studios s ON st.studio_id = s.id
    LEFT JOIN bookings b ON b.showtime_id = st.id 
        AND b.payment_status != 'CANCELLED'
    LEFT JOIN booking_seats bs ON bs.booking_id = b.id
    WHERE st.movie_id = p_movie_id
        AND st.show_date = p_show_date
        AND st.status = 'AVAILABLE'
    GROUP BY st.id
    ORDER BY st.show_time;
END //
DELIMITER ;

-- Procedure 6: Cancel Booking
DELIMITER //
DROP PROCEDURE IF EXISTS CancelBooking//
CREATE PROCEDURE CancelBooking(IN p_booking_code VARCHAR(50))
BEGIN
    DECLARE v_booking_id INT;
    DECLARE v_payment_status VARCHAR(20);
    
    SELECT id, payment_status INTO v_booking_id, v_payment_status
    FROM bookings 
    WHERE booking_code = p_booking_code;
    
    IF v_booking_id IS NULL THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Booking tidak ditemukan';
    END IF;
    
    IF v_payment_status = 'PAID' THEN
        SIGNAL SQLSTATE '45000' SET MESSAGE_TEXT = 'Booking yang sudah dibayar tidak dapat dibatalkan';
    END IF;
    
    UPDATE bookings 
    SET payment_status = 'CANCELLED'
    WHERE id = v_booking_id;
    
    SELECT 'Success' as status, 'Booking berhasil dibatalkan' as message;
END //
DELIMITER ;

-- Procedure 7: Get User Booking History
DELIMITER //
DROP PROCEDURE IF EXISTS GetUserBookingHistory//
CREATE PROCEDURE GetUserBookingHistory(IN p_user_id INT)
BEGIN
    SELECT 
        b.id,
        b.booking_code,
        b.total_price,
        b.payment_status,
        b.created_at,
        m.title as movie_title,
        m.poster_url,
        st.show_date,
        st.show_time,
        s.studio_name,
        GROUP_CONCAT(se.seat_code ORDER BY se.seat_row, se.seat_col) as seats
    FROM bookings b
    JOIN showtimes st ON b.showtime_id = st.id
    JOIN movies m ON st.movie_id = m.id
    JOIN studios s ON st.studio_id = s.id
    JOIN booking_seats bs ON b.id = bs.booking_id
    JOIN seats se ON bs.seat_id = se.id
    WHERE b.user_id = p_user_id
    GROUP BY b.id
    ORDER BY b.created_at DESC;
END //
DELIMITER ;

-- Procedure 8: Generate Seat Layout for Studio
DELIMITER //
DROP PROCEDURE IF EXISTS GenerateSeatLayout//
CREATE PROCEDURE GenerateSeatLayout(
    IN p_studio_id INT,
    IN p_total_rows INT,
    IN p_total_cols INT
)
BEGIN
    DECLARE v_row_counter INT DEFAULT 1;
    DECLARE v_col_counter INT;
    DECLARE v_row_letter CHAR(1);
    DECLARE v_seat_code VARCHAR(10);
    
    -- Delete existing seats for this studio
    DELETE FROM seats WHERE studio_id = p_studio_id;
    
    -- Generate seats
    WHILE v_row_counter <= p_total_rows DO
        SET v_row_letter = CHAR(64 + v_row_counter); -- A=65, B=66, dst
        SET v_col_counter = 1;
        
        WHILE v_col_counter <= p_total_cols DO
            SET v_seat_code = CONCAT(v_row_letter, v_col_counter);
            
            INSERT INTO seats (studio_id, seat_code, seat_row, seat_col, seat_status)
            VALUES (p_studio_id, v_seat_code, v_row_letter, v_col_counter, 'AVAILABLE');
            
            SET v_col_counter = v_col_counter + 1;
        END WHILE;
        
        SET v_row_counter = v_row_counter + 1;
    END WHILE;
    
    -- Update total_seats in studios table
    UPDATE studios 
    SET total_seats = p_total_rows * p_total_cols,
        total_rows = p_total_rows,
        total_cols = p_total_cols
    WHERE id = p_studio_id;
    
    SELECT 'Success' as status, 
           CONCAT('Generated ', p_total_rows * p_total_cols, ' seats') as message;
END //
DELIMITER ;

-- Procedure 9: Get Dashboard Statistics
DELIMITER //
DROP PROCEDURE IF EXISTS GetDashboardStatistics//
CREATE PROCEDURE GetDashboardStatistics()
BEGIN
    SELECT 
        (SELECT COUNT(*) FROM movies WHERE status = 'NOW_SHOWING') as total_movies,
        (SELECT COUNT(*) FROM users WHERE role = 'USER') as total_users,
        (SELECT COUNT(*) FROM bookings WHERE payment_status = 'PAID' 
         AND DATE(created_at) = CURDATE()) as today_bookings,
        (SELECT COALESCE(SUM(total_price), 0) FROM bookings 
         WHERE payment_status = 'PAID' 
         AND DATE(created_at) = CURDATE()) as today_revenue,
        (SELECT COALESCE(SUM(total_price), 0) FROM bookings 
         WHERE payment_status = 'PAID' 
         AND MONTH(created_at) = MONTH(CURDATE())
         AND YEAR(created_at) = YEAR(CURDATE())) as monthly_revenue;
END //
DELIMITER ;

-- ============================================
-- SAMPLE DATA
-- ============================================

-- Insert sample users
INSERT INTO users (username, email, password, full_name, phone, role) VALUES
('admin', 'admin@tioskop.com', '$2a$10$YourHashedPasswordHere', 'Admin Tioskop', '081234567890', 'ADMIN'),
('john_doe', 'john@example.com', '$2a$10$YourHashedPasswordHere', 'John Doe', '081234567891', 'USER'),
('jane_smith', 'jane@example.com', '$2a$10$YourHashedPasswordHere', 'Jane Smith', '081234567892', 'USER');

-- Insert sample movies
INSERT INTO movies (title, description, duration, genre, rating, release_date, poster_url, director, cast, status) VALUES
('Avengers: Endgame', 'The epic conclusion to the Infinity Saga', 181, 'Action, Adventure, Sci-Fi', 'PG-13', '2024-04-26', 'https://example.com/avengers.jpg', 'Russo Brothers', 'Robert Downey Jr., Chris Evans, Scarlett Johansson', 'NOW_SHOWING'),
('The Batman', 'The Dark Knight returns', 176, 'Action, Crime, Drama', 'PG-13', '2024-03-04', 'https://example.com/batman.jpg', 'Matt Reeves', 'Robert Pattinson, Zoë Kravitz', 'NOW_SHOWING'),
('Spider-Man: No Way Home', 'The multiverse unleashed', 148, 'Action, Adventure, Fantasy', 'PG-13', '2024-12-17', 'https://example.com/spiderman.jpg', 'Jon Watts', 'Tom Holland, Zendaya', 'NOW_SHOWING'),
('Dune: Part Two', 'The journey continues', 166, 'Adventure, Drama, Sci-Fi', 'PG-13', '2025-03-01', 'https://example.com/dune.jpg', 'Denis Villeneuve', 'Timothée Chalamet, Zendaya', 'COMING_SOON');

-- Insert sample studios
INSERT INTO studios (studio_name, studio_type, total_seats, total_rows, total_cols) VALUES
('Studio 1', 'REGULAR', 100, 10, 10),
('Studio 2', 'PREMIERE', 50, 5, 10),
('Studio 3', 'IMAX', 150, 15, 10),
('Studio 4', 'DOLBY', 80, 8, 10);

-- Generate seats for studios
CALL GenerateSeatLayout(1, 10, 10);
CALL GenerateSeatLayout(2, 5, 10);
CALL GenerateSeatLayout(3, 15, 10);
CALL GenerateSeatLayout(4, 8, 10);

-- Insert sample showtimes
INSERT INTO showtimes (movie_id, studio_id, show_date, show_time, price, status) VALUES
-- Avengers: Endgame
(1, 1, '2024-11-28', '10:00:00', 50000, 'AVAILABLE'),
(1, 1, '2024-11-28', '13:30:00', 50000, 'AVAILABLE'),
(1, 3, '2024-11-28', '16:00:00', 75000, 'AVAILABLE'),
(1, 3, '2024-11-28', '19:30:00', 75000, 'AVAILABLE'),
-- The Batman
(2, 2, '2024-11-28', '11:00:00', 60000, 'AVAILABLE'),
(2, 2, '2024-11-28', '14:30:00', 60000, 'AVAILABLE'),
(2, 4, '2024-11-28', '18:00:00', 70000, 'AVAILABLE'),
-- Spider-Man
(3, 1, '2024-11-28', '12:00:00', 50000, 'AVAILABLE'),
(3, 4, '2024-11-28', '15:30:00', 70000, 'AVAILABLE'),
(3, 4, '2024-11-28', '21:00:00', 70000, 'AVAILABLE');

-- Insert sample bookings
INSERT INTO bookings (user_id, showtime_id, booking_code, total_price, payment_status, payment_method, payment_date) VALUES
(2, 1, 'BK001-20241128-001', 150000, 'PAID', 'Credit Card', '2024-11-27 10:30:00'),
(3, 2, 'BK002-20241128-002', 100000, 'PAID', 'E-Wallet', '2024-11-27 14:15:00'),
(2, 5, 'BK003-20241128-003', 120000, 'PENDING', NULL, NULL);

-- Insert sample booking_seats
INSERT INTO booking_seats (booking_id, seat_id) VALUES
-- Booking 1 (3 seats)
(1, 1), (1, 2), (1, 3),
-- Booking 2 (2 seats)
(2, 15), (2, 16),
-- Booking 3 (2 seats)
(3, 501), (3, 502);

-- ============================================
-- VIEWS FOR REPORTING
-- ============================================

-- View: Active Showtimes
CREATE OR REPLACE VIEW vw_active_showtimes AS
SELECT 
    st.id,
    st.show_date,
    st.show_time,
    st.price,
    st.status,
    m.title as movie_title,
    m.rating,
    m.duration,
    s.studio_name,
    s.studio_type,
    s.total_seats,
    COUNT(DISTINCT bs.seat_id) as booked_seats,
    (s.total_seats - COUNT(DISTINCT bs.seat_id)) as available_seats
FROM showtimes st
JOIN movies m ON st.movie_id = m.id
JOIN studios s ON st.studio_id = s.id
LEFT JOIN bookings b ON b.showtime_id = st.id AND b.payment_status != 'CANCELLED'
LEFT JOIN booking_seats bs ON bs.booking_id = b.id
WHERE st.status = 'AVAILABLE'
GROUP BY st.id;

-- View: Today's Bookings
CREATE OR REPLACE VIEW vw_todays_bookings AS
SELECT 
    b.booking_code,
    b.total_price,
    b.payment_status,
    b.created_at,
    u.full_name,
    u.email,
    m.title as movie_title,
    st.show_date,
    st.show_time,
    s.studio_name
FROM bookings b
JOIN users u ON b.user_id = u.id
JOIN showtimes st ON b.showtime_id = st.id
JOIN movies m ON st.movie_id = m.id
JOIN studios s ON st.studio_id = s.id
WHERE DATE(b.created_at) = CURDATE();

-- ============================================
-- TRIGGERS
-- ============================================

-- Trigger: Update showtime status when fully booked
DELIMITER //
DROP TRIGGER IF EXISTS after_booking_insert//
CREATE TRIGGER after_booking_insert
AFTER INSERT ON booking_seats
FOR EACH ROW
BEGIN
    DECLARE v_showtime_id INT;
    DECLARE v_studio_id INT;
    DECLARE v_total_seats INT;
    DECLARE v_booked_seats INT;
    
    -- Get showtime info
    SELECT b.showtime_id, st.studio_id
    INTO v_showtime_id, v_studio_id
    FROM bookings b
    JOIN showtimes st ON b.showtime_id = st.id
    WHERE b.id = NEW.booking_id;
    
    -- Get total seats
    SELECT total_seats INTO v_total_seats
    FROM studios WHERE id = v_studio_id;
    
    -- Count booked seats
    SELECT COUNT(DISTINCT bs.seat_id) INTO v_booked_seats
    FROM booking_seats bs
    JOIN bookings b ON b.id = bs.booking_id
    WHERE b.showtime_id = v_showtime_id
        AND b.payment_status != 'CANCELLED';
    
    -- Update status if full
    IF v_booked_seats >= v_total_seats THEN
        UPDATE showtimes 
        SET status = 'FULL'
        WHERE id = v_showtime_id;
    END IF;
END //
DELIMITER ;

-- Trigger: Update showtime status when booking cancelled
DELIMITER //
DROP TRIGGER IF EXISTS after_booking_cancel//
CREATE TRIGGER after_booking_cancel
AFTER UPDATE ON bookings
FOR EACH ROW
BEGIN
    IF NEW.payment_status = 'CANCELLED' AND OLD.payment_status != 'CANCELLED' THEN
        UPDATE showtimes 
        SET status = 'AVAILABLE'
        WHERE id = NEW.showtime_id AND status = 'FULL';
    END IF;
END //
DELIMITER ;

-- ============================================
-- GRANT PERMISSIONS (adjust as needed)
-- ============================================

-- CREATE USER 'tioskop_user'@'localhost' IDENTIFIED BY 'your_password';
-- GRANT ALL PRIVILEGES ON tioskop_db.* TO 'tioskop_user'@'localhost';
-- FLUSH PRIVILEGES;

SELECT 'Database tioskop_db created successfully!' as message;