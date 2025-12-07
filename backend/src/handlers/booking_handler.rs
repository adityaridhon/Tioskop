use crate::middleware::auth::AuthUser;
use crate::models::*;
use crate::entities::{Booking, BookingsEntity, BookingSeatsEntity};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, QueryOrder, ColumnTrait, QueryFilter, FromQueryResult, PaginatorTrait};
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
pub async fn get_all_bookings(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Booking>>> {
    use crate::entities::bookings::Column;

    BookingsEntity::find()
        .order_by_desc(Column::CreatedAt)
        .all(&db)
        .await
        .map(|bookings| Json(ApiResponse::success("Berhasil mengambil semua bookings", bookings)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get booking by ID dengan detail seats - Functional Programming approach
pub async fn get_booking_by_id(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<BookingDetail>> {
    // Fetch booking
    let booking_result = BookingsEntity::find_by_id(id).one(&db).await;

    match booking_result {
        Ok(Some(booking)) => {
            // Fetch booking seats dengan raw SQL (complex JOIN)
            use sea_orm::Statement;
            
            let seats_query = Statement::from_string(
                sea_orm::DatabaseBackend::MySql,
                format!(
                    "SELECT bs.seat_id, s.seat_code, bs.price \
                    FROM booking_seats bs \
                    JOIN seats s ON bs.seat_id = s.id \
                    WHERE bs.booking_id = {}",
                    id
                ),
            );

            #[derive(Debug, FromQueryResult)]
            struct SeatRow {
                seat_id: i64,
                seat_code: String,
                price: Option<rust_decimal::Decimal>,
            }

            let seats_result = SeatRow::find_by_statement(seats_query)
                .all(&db)
                .await
                .map(|rows| {
                    rows.into_iter()
                        .map(|row| BookingSeatDetail {
                            seat_id: row.seat_id,
                            seat_code: row.seat_code,
                            price: row.price,
                        })
                        .collect::<Vec<_>>()
                });

            match seats_result {
                Ok(seats) => {
                    let detail = BookingDetail {
                        id: booking.id,
                        user_id: booking.user_id,
                        showtime_id: booking.showtime_id,
                        booking_code: Some(booking.booking_code),
                        total_price: booking.total_price,
                        payment_status: Some(booking.payment_status),
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
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i64>,
) -> Json<ApiResponse<Vec<Booking>>> {
    use crate::entities::bookings::Column;

    BookingsEntity::find()
        .filter(Column::UserId.eq(user_id))
        .order_by_desc(Column::CreatedAt)
        .all(&db)
        .await
        .map(|bookings| Json(ApiResponse::success("Berhasil mengambil bookings user", bookings)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Create booking - Functional Programming approach (Protected with JWT)
pub async fn create_booking(
    AuthUser(authenticated_user_id): AuthUser,
    State(db): State<DatabaseConnection>,
    Json(payload): Json<CreateBookingRequest>,
) -> Json<ApiResponse<BookingDetail>> {
    // Use authenticated user_id instead of payload.user_id for security
    let user_id = authenticated_user_id;

    // Validasi: cek apakah seats sudah dibooking dengan SeaORM
    use crate::entities::{bookings::Column as BookingCol, booking_seats::Column as BookingSeatCol};

    let booked_count = BookingSeatsEntity::find()
        .filter(BookingSeatCol::SeatId.is_in(payload.seat_ids.clone()))
        .inner_join(BookingsEntity)
        .filter(BookingCol::ShowtimeId.eq(payload.showtime_id))
        .filter(BookingCol::PaymentStatus.ne("CANCELLED"))
        .count(&db)
        .await;

    match booked_count {
        Ok(count) if count > 0 => {
            return Json(ApiResponse::error("Beberapa kursi sudah dibooking"));
        }
        Err(e) => {
            return Json(ApiResponse::error(&format!("Error checking seats: {}", e)));
        }
        _ => {}
    }

    // Ambil harga showtime dengan SeaORM
    use crate::entities::ShowtimesEntity;
    
    let showtime_result = ShowtimesEntity::find_by_id(payload.showtime_id)
        .one(&db)
        .await;

    match showtime_result {
        Ok(Some(showtime)) => {
            let price = showtime.price.unwrap_or_default();
            
            // Calculate total price dengan functional approach
            let total_price = price * rust_decimal::Decimal::from(payload.seat_ids.len() as i32);
            let booking_code = generate_booking_code();

            // Insert booking dengan SeaORM ActiveModel
            use crate::entities::bookings::ActiveModel as BookingActiveModel;
            
            let new_booking = BookingActiveModel {
                user_id: Set(Some(user_id)),
                showtime_id: Set(Some(payload.showtime_id)),
                booking_code: Set(booking_code.clone()),
                total_price: Set(Some(total_price)),
                payment_status: Set("PENDING".to_string()),
                ..Default::default()
            };

            match new_booking.insert(&db).await {
                Ok(booking) => {
                    let booking_id = booking.id;

                    // Insert booking_seats dengan SeaORM (immutable functional approach)
                    use crate::entities::booking_seats::ActiveModel as BookingSeatActiveModel;
                    use crate::entities::SeatsEntity;
                    
                    for seat_id in &payload.seat_ids {
                        let booking_seat = BookingSeatActiveModel {
                            booking_id: Set(Some(booking_id)),
                            seat_id: Set(Some(*seat_id)),
                            price: Set(Some(price)),
                            ..Default::default()
                        };
                        booking_seat.insert(&db).await.ok();
                    }

                    // Update seat status menjadi 'booked' dengan SeaORM
                    use crate::entities::seats::ActiveModel as SeatActiveModel;
                    
                    for seat_id in &payload.seat_ids {
                        if let Ok(Some(seat)) = SeatsEntity::find_by_id(*seat_id).one(&db).await {
                            let mut seat_active: SeatActiveModel = seat.into();
                            seat_active.seat_status = Set(Some("booked".to_string()));
                            seat_active.update(&db).await.ok();
                        }
                    }

                    // Fetch created booking seats dengan SeaORM raw query
                    use sea_orm::Statement;
                    
                    let seats_query = Statement::from_string(
                        sea_orm::DatabaseBackend::MySql,
                        format!(
                            "SELECT bs.seat_id, s.seat_code, bs.price \
                            FROM booking_seats bs \
                            JOIN seats s ON bs.seat_id = s.id \
                            WHERE bs.booking_id = {}",
                            booking_id
                        ),
                    );

                    #[derive(Debug, FromQueryResult)]
                    struct SeatRow {
                        seat_id: i64,
                        seat_code: String,
                        price: Option<rust_decimal::Decimal>,
                    }

                    let seats_result = SeatRow::find_by_statement(seats_query)
                        .all(&db)
                        .await
                        .map(|rows| {
                            rows.into_iter()
                                .map(|row| BookingSeatDetail {
                                    seat_id: row.seat_id,
                                    seat_code: row.seat_code,
                                    price: row.price,
                                })
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    let booking_detail_result = BookingsEntity::find_by_id(booking_id).one(&db).await;

                    match booking_detail_result {
                        Ok(Some(booking)) => {
                            let detail = BookingDetail {
                                id: booking.id,
                                user_id: booking.user_id,
                                showtime_id: booking.showtime_id,
                                booking_code: Some(booking.booking_code),
                                total_price: booking.total_price,
                                payment_status: Some(booking.payment_status),
                                created_at: booking.created_at,
                                seats: seats_result,
                            };

                            Json(ApiResponse::success("Berhasil membuat booking", detail))
                        }
                        Ok(None) => Json(ApiResponse::error("Booking tidak ditemukan setelah dibuat")),
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
    State(db): State<DatabaseConnection>,
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

    // Update status dengan SeaORM
    let booking_result = BookingsEntity::find_by_id(id).one(&db).await;

    match booking_result {
        Ok(Some(booking)) => {
            use crate::entities::bookings::ActiveModel;
            
            let mut active_booking: ActiveModel = booking.into();
            active_booking.payment_status = Set(payload.payment_status);
            
            active_booking
                .update(&db)
                .await
                .map(|updated| Json(ApiResponse::success("Berhasil update status payment", updated)))
                .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to update: {}", e))))
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Booking dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Cancel booking - Functional Programming approach
pub async fn cancel_booking(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i64>,
) -> Json<ApiResponse<Booking>> {
    // Get seat IDs sebelum cancel dengan SeaORM
    use crate::entities::booking_seats::Column as BookingSeatCol;
    
    let seat_ids_result = BookingSeatsEntity::find()
        .filter(BookingSeatCol::BookingId.eq(id))
        .all(&db)
        .await;

    // Update booking status ke CANCELLED dengan SeaORM
    let booking_result = BookingsEntity::find_by_id(id).one(&db).await;

    match booking_result {
        Ok(Some(booking)) => {
            use crate::entities::bookings::ActiveModel;
            use crate::entities::{SeatsEntity, seats::ActiveModel as SeatActiveModel};
            
            let mut active_booking: ActiveModel = booking.into();
            active_booking.payment_status = Set("CANCELLED".to_string());
            
            // Update booking status
            let updated_booking = active_booking.update(&db).await;

            // Kembalikan status kursi menjadi 'available' dengan SeaORM (immutable)
            if let Ok(booking_seats) = seat_ids_result {
                for bs in booking_seats {
                    if let Some(seat_id) = bs.seat_id {
                        if let Ok(Some(seat)) = SeatsEntity::find_by_id(seat_id).one(&db).await {
                            let mut seat_active: SeatActiveModel = seat.into();
                            seat_active.seat_status = Set(Some("available".to_string()));
                            seat_active.update(&db).await.ok();
                        }
                    }
                }
            }

            updated_booking
                .map(|b| Json(ApiResponse::success("Berhasil cancel booking", b)))
                .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Failed to cancel: {}", e))))
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Booking dengan id {} tidak ditemukan",
            id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get booked seats by showtime
pub async fn get_booked_seats_by_showtime(
    State(db): State<DatabaseConnection>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<String>>> {
    use sea_orm::Statement;
    
    let query = Statement::from_string(
        sea_orm::DatabaseBackend::MySql,
        format!(
            "SELECT s.seat_code \
            FROM booking_seats bs \
            JOIN bookings b ON bs.booking_id = b.id \
            JOIN seats s ON bs.seat_id = s.id \
            WHERE b.showtime_id = {} AND b.payment_status != 'CANCELLED'",
            showtime_id
        ),
    );

    #[derive(Debug, FromQueryResult)]
    struct SeatCodeRow {
        seat_code: String,
    }

    let seats_result = SeatCodeRow::find_by_statement(query)
        .all(&db)
        .await
        .map(|rows| rows.into_iter().map(|r| r.seat_code).collect::<Vec<_>>());

    match seats_result {
        Ok(seats) => Json(ApiResponse::success(
            "Berhasil mengambil kursi yang dibooking",
            seats,
        )),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}
