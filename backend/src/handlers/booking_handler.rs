use crate::middleware::auth::AuthUser;
use crate::models::*;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::MySqlPool;
use std::time::{SystemTime, UNIX_EPOCH};

// Generate unique booking code - Pure function
fn generate_booking_code() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("BK{}", timestamp)
}

// Get all bookings - Functional Programming approach
pub async fn get_all_bookings(State(pool): State<MySqlPool>) -> Json<ApiResponse<Vec<Booking>>> {
    sqlx::query_as::<_, Booking>(
        "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map(|bookings| Json(ApiResponse::success("Berhasil mengambil semua bookings", bookings)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get booking by ID dengan detail seats - Functional Programming approach
pub async fn get_booking_by_id(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<BookingDetail>> {
    // Fetch booking
    let booking_result = sqlx::query_as::<_, Booking>(
        "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings WHERE id = ?"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await;

    match booking_result {
        Ok(Some(booking)) => {
            // Fetch booking seats dengan functional approach
            let seats_result = sqlx::query_as::<_, (i64, String, Option<rust_decimal::Decimal>)>(
                "SELECT bs.seat_id, s.seat_code, bs.price 
                FROM booking_seats bs 
                JOIN seats s ON bs.seat_id = s.id 
                WHERE bs.booking_id = ?",
            )
            .bind(id)
            .fetch_all(&pool)
            .await
            .map(|rows| {
                rows.into_iter()
                    .map(|(seat_id, seat_code, price)| BookingSeatDetail {
                        seat_id,
                        seat_code,
                        price,
                    })
                    .collect::<Vec<_>>()
            });

            match seats_result {
                Ok(seats) => {
                    let detail = BookingDetail {
                        id: booking.id,
                        user_id: booking.user_id,
                        showtime_id: booking.showtime_id,
                        booking_code: booking.booking_code,
                        total_price: booking.total_price,
                        payment_status: booking.payment_status,
                        created_at: booking.created_at,
                        seats,
                    };
                    Json(ApiResponse::success(
                        "Berhasil mengambil detail booking",
                        detail,
                    ))
                }
                Err(e) => Json(ApiResponse::error(&format!("Failed to fetch seats: {}", e))),
            }
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Booking dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get bookings by user_id - Functional Programming approach
pub async fn get_bookings_by_user(
    State(pool): State<MySqlPool>,
    Path(user_id): Path<i64>,
) -> Json<ApiResponse<Vec<Booking>>> {
    sqlx::query_as::<_, Booking>(
        "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings WHERE user_id = ? ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map(|bookings| Json(ApiResponse::success("Berhasil mengambil bookings user", bookings)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Create booking - Functional Programming approach (Protected with JWT)
pub async fn create_booking(
    AuthUser(authenticated_user_id): AuthUser,
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateBookingRequest>,
) -> Json<ApiResponse<BookingDetail>> {
    // Use authenticated user_id instead of payload.user_id for security
    let user_id = authenticated_user_id;

    // Validasi: cek apakah seats sudah dibooking
    let seat_ids_str = payload
        .seat_ids
        .iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let booked_seats_check = sqlx::query_scalar::<_, i64>(&format!(
        "SELECT COUNT(*) FROM booking_seats bs
            JOIN bookings b ON bs.booking_id = b.id
            WHERE bs.seat_id IN ({}) 
            AND b.showtime_id = ?
            AND b.payment_status != 'CANCELLED'",
        seat_ids_str
    ))
    .bind(payload.showtime_id)
    .fetch_one(&pool)
    .await;

    match booked_seats_check {
        Ok(count) if count > 0 => {
            return Json(ApiResponse::error("Beberapa kursi sudah dibooking"));
        }
        Err(e) => {
            return Json(ApiResponse::error(&format!("Error checking seats: {}", e)));
        }
        _ => {}
    }

    // Ambil harga showtime
    let price_result =
        sqlx::query_scalar::<_, rust_decimal::Decimal>("SELECT price FROM showtimes WHERE id = ?")
            .bind(payload.showtime_id)
            .fetch_optional(&pool)
            .await;

    match price_result {
        Ok(Some(price)) => {
            // Calculate total price dengan functional approach
            let total_price = price * rust_decimal::Decimal::from(payload.seat_ids.len() as i32);
            let booking_code = generate_booking_code();

            // Insert booking
            let insert_result = sqlx::query(
                "INSERT INTO bookings (user_id, showtime_id, booking_code, total_price, payment_status) VALUES (?, ?, ?, ?, 'PENDING')"
            )
            .bind(user_id)
            .bind(payload.showtime_id)
            .bind(&booking_code)
            .bind(total_price)
            .execute(&pool)
            .await;

            match insert_result {
                Ok(result) => {
                    let booking_id = result.last_insert_id() as i64;

                    // Insert booking_seats dengan functional approach
                    let seats_insert_futures = payload.seat_ids.iter()
                        .map(|seat_id| {
                            sqlx::query(
                                "INSERT INTO booking_seats (booking_id, seat_id, price) VALUES (?, ?, ?)"
                            )
                            .bind(booking_id)
                            .bind(seat_id)
                            .bind(price)
                            .execute(&pool)
                        });

                    // Execute all inserts
                    for future in seats_insert_futures {
                        future.await.ok();
                    }

                    // Update seat status menjadi 'booked' dengan functional approach
                    for seat_id in payload.seat_ids.iter() {
                        sqlx::query("UPDATE seats SET seat_status = 'booked' WHERE id = ?")
                            .bind(seat_id)
                            .execute(&pool)
                            .await
                            .ok();
                    }

                    // Fetch created booking dengan seats
                    let booking_detail_result = sqlx::query_as::<_, Booking>(
                        "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings WHERE id = ?"
                    )
                    .bind(booking_id)
                    .fetch_one(&pool)
                    .await;

                    match booking_detail_result {
                        Ok(booking) => {
                            let seats_result =
                                sqlx::query_as::<_, (i64, String, Option<rust_decimal::Decimal>)>(
                                    "SELECT bs.seat_id, s.seat_code, bs.price 
                                FROM booking_seats bs 
                                JOIN seats s ON bs.seat_id = s.id 
                                WHERE bs.booking_id = ?",
                                )
                                .bind(booking_id)
                                .fetch_all(&pool)
                                .await
                                .map(|rows| {
                                    rows.into_iter()
                                        .map(|(seat_id, seat_code, price)| BookingSeatDetail {
                                            seat_id,
                                            seat_code,
                                            price,
                                        })
                                        .collect::<Vec<_>>()
                                })
                                .unwrap_or_default();

                            let detail = BookingDetail {
                                id: booking.id,
                                user_id: booking.user_id,
                                showtime_id: booking.showtime_id,
                                booking_code: booking.booking_code,
                                total_price: booking.total_price,
                                payment_status: booking.payment_status,
                                created_at: booking.created_at,
                                seats: seats_result,
                            };

                            Json(ApiResponse::success("Berhasil membuat booking", detail))
                        }
                        Err(e) => Json(ApiResponse::error(&format!(
                            "Failed to fetch created booking: {}",
                            e
                        ))),
                    }
                }
                Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
            }
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Showtime dengan id {} tidak ditemukan",
            payload.showtime_id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Update payment status - Functional Programming approach
pub async fn update_payment_status(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
    Json(payload): Json<UpdatePaymentStatusRequest>,
) -> Json<ApiResponse<Booking>> {
    // Validasi payment status
    let valid_statuses = vec!["PENDING", "PAID", "CANCELLED"];
    if !valid_statuses.contains(&payload.payment_status.as_str()) {
        return Json(ApiResponse::error(
            "Status payment tidak valid. Harus PENDING, PAID, atau CANCELLED",
        ));
    }

    // Update status
    let update_result = sqlx::query("UPDATE bookings SET payment_status = ? WHERE id = ?")
        .bind(&payload.payment_status)
        .bind(id)
        .execute(&pool)
        .await;

    match update_result {
        Ok(result) if result.rows_affected() > 0 => {
            // Fetch updated booking
            sqlx::query_as::<_, Booking>(
                "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings WHERE id = ?"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .map(|booking| Json(ApiResponse::success("Berhasil update status payment", booking)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch updated booking: {}", e))))
        }
        Ok(_) => Json(ApiResponse::error(&format!(
            "Booking dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Cancel booking - Functional Programming approach
pub async fn cancel_booking(
    State(pool): State<MySqlPool>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Booking>> {
    // Get seat IDs sebelum cancel untuk kembalikan status
    let seats_result =
        sqlx::query_scalar::<_, i64>("SELECT seat_id FROM booking_seats WHERE booking_id = ?")
            .bind(id)
            .fetch_all(&pool)
            .await;

    // Update status ke CANCELLED
    let update_result =
        sqlx::query("UPDATE bookings SET payment_status = 'CANCELLED' WHERE id = ?")
            .bind(id)
            .execute(&pool)
            .await;

    match update_result {
        Ok(result) if result.rows_affected() > 0 => {
            // Kembalikan status kursi menjadi 'available'
            if let Ok(seat_ids) = seats_result {
                for seat_id in seat_ids.iter() {
                    sqlx::query("UPDATE seats SET seat_status = 'available' WHERE id = ?")
                        .bind(seat_id)
                        .execute(&pool)
                        .await
                        .ok();
                }
            }

            sqlx::query_as::<_, Booking>(
                "SELECT id, user_id, showtime_id, booking_code, total_price, payment_status, CAST(created_at AS DATETIME) as created_at FROM bookings WHERE id = ?"
            )
            .bind(id)
            .fetch_one(&pool)
            .await
            .map(|booking| Json(ApiResponse::success("Berhasil cancel booking", booking)))
            .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to fetch cancelled booking: {}", e))))
        }
        Ok(_) => Json(ApiResponse::error(&format!(
            "Booking dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get booked seats by showtime
pub async fn get_booked_seats_by_showtime(
    State(pool): State<MySqlPool>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<String>>> {
    let seats_result = sqlx::query_scalar::<_, String>(
        "SELECT s.seat_code 
        FROM booking_seats bs
        JOIN bookings b ON bs.booking_id = b.id
        JOIN seats s ON bs.seat_id = s.id
        WHERE b.showtime_id = ? AND b.payment_status != 'CANCELLED'",
    )
    .bind(showtime_id)
    .fetch_all(&pool)
    .await;

    match seats_result {
        Ok(seats) => Json(ApiResponse::success(
            "Berhasil mengambil kursi yang dibooking",
            seats,
        )),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}
