use axum::{extract::{Path, Extension}, Json};
use sqlx::MySqlPool;
use crate::models::*;

// Get all seats
pub async fn get_all_seats(
    Extension(pool): Extension<MySqlPool>,
) -> Json<ApiResponse<Vec<Seat>>> {
    sqlx::query_as::<_, Seat>(
        "SELECT id, studio_id, seat_code, seat_row, seat_col, seat_status FROM seats"
    )
    .fetch_all(&pool)
    .await
    .map(|seats| Json(ApiResponse::success("Berhasil mengambil semua seats", seats)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get seats by studio_id
pub async fn get_seats_by_studio(
    Extension(pool): Extension<MySqlPool>,
    Path(studio_id): Path<i64>,
) -> Json<ApiResponse<Vec<Seat>>> {
    sqlx::query_as::<_, Seat>(
        "SELECT id, studio_id, seat_code, seat_row, seat_col, seat_status FROM seats WHERE studio_id = ? ORDER BY seat_row, seat_col"
    )
    .bind(studio_id)
    .fetch_all(&pool)
    .await
    .map(|seats| Json(ApiResponse::success("Berhasil mengambil seats untuk studio ini", seats)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get seats by showtime
pub async fn get_seats_by_showtime(
    Extension(pool): Extension<MySqlPool>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<SeatWithBookingStatus>>> {
    // Query seats dengan LEFT JOIN ke booking_seats untuk cek status booking
    sqlx::query_as::<_, (i64, i64, String, Option<i32>, Option<i32>, Option<String>, Option<i64>)>(
        "SELECT 
            s.id, 
            s.studio_id,
            s.seat_code, 
            s.seat_row, 
            s.seat_col,
            s.seat_status,
            bs.booking_id
        FROM seats s
        JOIN showtimes st ON s.studio_id = st.studio_id
        LEFT JOIN booking_seats bs ON bs.seat_id = s.id 
            AND bs.booking_id IN (
                SELECT b.id FROM bookings b WHERE b.showtime_id = st.id
            )
        WHERE st.id = ?
        ORDER BY s.seat_row, s.seat_col"
    )
    .bind(showtime_id)
    .fetch_all(&pool)
    .await
    .map(|rows| {
        // Transform data dengan functional approach
        rows.into_iter()
            .map(|(id, studio_id, seat_code, seat_row, seat_col, seat_status, booking_id)| {
                SeatWithBookingStatus {
                    id,
                    studio_id,
                    seat_code,
                    seat_row,
                    seat_col,
                    seat_status,
                    is_booked: booking_id.is_some(),
                    booking_id,
                }
            })
            .collect::<Vec<_>>()
    })
    .map(|seats| Json(ApiResponse::success("Berhasil mengambil seats dengan status booking", seats)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get available seats only for a showtime
pub async fn get_available_seats_by_showtime(
    Extension(pool): Extension<MySqlPool>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<Seat>>> {
    sqlx::query_as::<_, Seat>(
        "SELECT DISTINCT
            s.id, 
            s.studio_id,
            s.seat_code, 
            s.seat_row, 
            s.seat_col,
            s.seat_status
        FROM seats s
        JOIN showtimes st ON s.studio_id = st.studio_id
        LEFT JOIN booking_seats bs ON bs.seat_id = s.id 
            AND bs.booking_id IN (
                SELECT b.id FROM bookings b WHERE b.showtime_id = st.id
            )
        WHERE st.id = ? 
            AND bs.booking_id IS NULL
            AND s.seat_status = 'AVAILABLE'
        ORDER BY s.seat_row, s.seat_col"
    )
    .bind(showtime_id)
    .fetch_all(&pool)
    .await
    .map(|seats| Json(ApiResponse::success("Berhasil mengambil seats yang tersedia", seats)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Generate seats studio 
pub async fn generate_seats_for_studio(
    Extension(pool): Extension<MySqlPool>,
    Json(payload): Json<GenerateSeatsRequest>,
) -> Json<ApiResponse<GenerateSeatsResponse>> {
    let studio_check = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM studios WHERE id = ?"
    )
    .bind(payload.studio_id)
    .fetch_one(&pool)
    .await;

    match studio_check {
        Ok(count) if count > 0 => {
            let seat_codes: Vec<(String, i32, i32)> = (1..=payload.rows)
                .flat_map(|row| {
                    let row_letter = char::from((b'A' + (row - 1) as u8)).to_string();
                    (1..=payload.seats_per_row)
                        .map(move |col| {
                            (format!("{}{}", row_letter, col), row, col)
                        })
                        .collect::<Vec<_>>()
                })
                .collect();

            let mut total_inserted = 0;
            for (seat_code, row, col) in seat_codes {
                let result = sqlx::query(
                    "INSERT INTO seats (studio_id, seat_code, seat_row, seat_col, seat_status) VALUES (?, ?, ?, ?, 'AVAILABLE')"
                )
                .bind(payload.studio_id)
                .bind(&seat_code)
                .bind(row)
                .bind(col)
                .execute(&pool)
                .await;

                if result.is_ok() {
                    total_inserted += 1;
                }
            }

            Json(ApiResponse::success(
                &format!("Berhasil generate {} kursi untuk studio {}", total_inserted, payload.studio_id),
                GenerateSeatsResponse {
                    studio_id: payload.studio_id,
                    total_seats_created: total_inserted,
                }
            ))
        },
        Ok(_) => Json(ApiResponse::error(&format!("Studio dengan id {} tidak ditemukan", payload.studio_id))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}
