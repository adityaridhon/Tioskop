use crate::entities::{Booking, BookingsEntity, BookingSeatsEntity, SeatsEntity, ShowtimesEntity};
use crate::models::booking::*;
use sea_orm::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum BookingError {
    NotFound(String),
    Database(String),
    Validation(String),
    SeatBooked,
}

impl std::fmt::Display for BookingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Validation(msg) => write!(f, "{}", msg),
            Self::SeatBooked => write!(f, "Kursi sudah dibooking"),
        }
    }
}

type Result<T> = std::result::Result<T, BookingError>;


/// Generate booking code
fn generate_booking_code() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("BK{}", timestamp)
}

/// Calculate total price 
fn calculate_total(price: rust_decimal::Decimal, seat_count: usize) -> rust_decimal::Decimal {
    price * rust_decimal::Decimal::from(seat_count as i32)
}

/// Validate payment status 
fn validate_payment_status(status: &str) -> Result<()> {
    match status {
        "PENDING" | "PAID" | "CANCELLED" => Ok(()),
        _ => Err(BookingError::Validation(format!("Invalid status: {}", status))),
    }
}

/// Build booking detail 
fn build_booking_detail(booking: Booking, seats: Vec<BookingSeatDetail>) -> BookingDetail {
    BookingDetail {
        id: booking.id,
        user_id: booking.user_id,
        showtime_id: booking.showtime_id,
        booking_code: Some(booking.booking_code),
        total_price: booking.total_price,
        payment_status: Some(booking.payment_status),
        created_at: booking.created_at,
        seats,
    }
}

/// Get all bookings
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Booking>> {
    use crate::entities::bookings::Column;
    
    BookingsEntity::find()
        .order_by_desc(Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))
}

/// Get booking by ID
pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<Booking> {
    BookingsEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?
        .ok_or_else(|| BookingError::NotFound(format!("Booking {} not found", id)))
}

/// Get bookings by user
pub async fn get_by_user(db: &DatabaseConnection, user_id: i64) -> Result<Vec<Booking>> {
    use crate::entities::bookings::Column;
    
    BookingsEntity::find()
        .filter(Column::UserId.eq(user_id))
        .order_by_desc(Column::CreatedAt)
        .all(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))
}

/// Check seats availability 
async fn check_seats_availability(
    db: &DatabaseConnection,
    seat_ids: &[i64],
    showtime_id: i64,
) -> Result<()> {
    use crate::entities::{bookings::Column as BC, booking_seats::Column as BSC};
    
    let count = BookingSeatsEntity::find()
        .filter(BSC::SeatId.is_in(seat_ids.to_vec()))
        .inner_join(BookingsEntity)
        .filter(BC::ShowtimeId.eq(showtime_id))
        .filter(BC::PaymentStatus.ne("CANCELLED"))
        .count(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?;
    
    if count > 0 {
        Err(BookingError::SeatBooked)
    } else {
        Ok(())
    }
}

/// Get showtime price
async fn get_showtime_price(db: &DatabaseConnection, showtime_id: i64) -> Result<rust_decimal::Decimal> {
    ShowtimesEntity::find_by_id(showtime_id)
        .one(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?
        .and_then(|st| st.price)
        .ok_or_else(|| BookingError::NotFound(format!("Showtime {} not found", showtime_id)))
}

/// Update seat status 
async fn update_seat_status(db: &DatabaseConnection, seat_id: i64, status: &str) -> Result<()> {
    use crate::entities::seats::ActiveModel as SeatActive;
    
    // Fetch seat
    let seat = SeatsEntity::find_by_id(seat_id)
        .one(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?
        .ok_or_else(|| BookingError::NotFound(format!("Seat {} not found", seat_id)))?;
    
    let active = SeatActive {
        id: Set(seat.id),
        studio_id: Set(seat.studio_id),
        seat_code: Set(seat.seat_code),
        seat_row: Set(seat.seat_row),      
        seat_col: Set(seat.seat_col),      
        seat_status: Set(Some(status.to_string())),
    };
    
    active
        .update(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?;
    
    Ok(())
}

/// Get booking seats with details
async fn get_booking_seats(db: &DatabaseConnection, booking_id: i64) -> Result<Vec<BookingSeatDetail>> {
    #[derive(FromQueryResult)]
    struct SeatRow {
        seat_id: i64,
        seat_code: String,
        price: Option<rust_decimal::Decimal>,
    }
    
    let query = Statement::from_string(
        DatabaseBackend::MySql,
        format!(
            "SELECT bs.seat_id, s.seat_code, bs.price \
            FROM booking_seats bs JOIN seats s ON bs.seat_id = s.id \
            WHERE bs.booking_id = {}",
            booking_id
        ),
    );
    
    SeatRow::find_by_statement(query)
        .all(db)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|r| BookingSeatDetail {
                    seat_id: r.seat_id,
                    seat_code: r.seat_code,
                    price: r.price,
                })
                .collect()
        })
        .map_err(|e| BookingError::Database(e.to_string()))
}

/// Get booking detail with seats 
pub async fn get_detail(db: &DatabaseConnection, id: i64) -> Result<BookingDetail> {
    let booking = get_by_id(db, id).await?;
    let seats = get_booking_seats(db, id).await?;
    
    Ok(build_booking_detail(booking, seats))
}

/// Create new booking 
pub async fn create(
    db: &DatabaseConnection,
    user_id: i64,
    request: CreateBookingRequest,
) -> Result<BookingDetail> {
    check_seats_availability(db, &request.seat_ids, request.showtime_id).await?;
    
    let price = get_showtime_price(db, request.showtime_id).await?;
    
    let total_price = calculate_total(price, request.seat_ids.len());
    
    let booking_code = generate_booking_code();
    
    use crate::entities::bookings::ActiveModel as BookingActive;
    
    let new_booking = BookingActive {
        user_id: Set(Some(user_id)),
        showtime_id: Set(Some(request.showtime_id)),
        booking_code: Set(booking_code),
        total_price: Set(Some(total_price)),
        payment_status: Set("PENDING".to_string()),
        ..Default::default()
    };
    
    let booking = new_booking
        .insert(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?;
    
    use crate::entities::booking_seats::ActiveModel as BookingSeatActive;
    
    for seat_id in &request.seat_ids {
        let bs = BookingSeatActive {
            booking_id: Set(Some(booking.id)),
            seat_id: Set(Some(*seat_id)),
            price: Set(Some(price)),
            ..Default::default()
        };
        bs.insert(db).await.ok();
        
        update_seat_status(db, *seat_id, "booked").await.ok();
    }
    
    get_detail(db, booking.id).await
}

/// Update payment status 
pub async fn update_payment(
    db: &DatabaseConnection,
    id: i64,
    status: String,
) -> Result<Booking> {
    validate_payment_status(&status)?;
    
    let booking = get_by_id(db, id).await?;
    
    use crate::entities::bookings::ActiveModel;
    
    let active = ActiveModel {
        id: Set(booking.id),
        user_id: Set(booking.user_id),
        showtime_id: Set(booking.showtime_id),
        booking_code: Set(booking.booking_code),
        total_price: Set(booking.total_price),
        payment_status: Set(status), 
        created_at: Set(booking.created_at),
    };
    
    active
        .update(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))
}

/// Cancel booking 
pub async fn cancel(db: &DatabaseConnection, id: i64) -> Result<Booking> {
    use crate::entities::booking_seats::Column as BSC;
    
    let seat_ids = BookingSeatsEntity::find()
        .filter(BSC::BookingId.eq(id))
        .all(db)
        .await
        .map_err(|e| BookingError::Database(e.to_string()))?
        .into_iter()
        .filter_map(|bs| bs.seat_id)
        .collect::<Vec<_>>();
    
    let booking = update_payment(db, id, "CANCELLED".to_string()).await?;
    
    for seat_id in seat_ids {
        update_seat_status(db, seat_id, "available").await.ok();
    }
    
    Ok(booking)
}

/// Get booked seats by showtime 
pub async fn get_booked_seats(db: &DatabaseConnection, showtime_id: i64) -> Result<Vec<String>> {
    #[derive(FromQueryResult)]
    struct SeatCode { seat_code: String }
    
    let query = Statement::from_string(
        DatabaseBackend::MySql,
        format!(
            "SELECT s.seat_code FROM booking_seats bs \
            JOIN bookings b ON bs.booking_id = b.id \
            JOIN seats s ON bs.seat_id = s.id \
            WHERE b.showtime_id = {} AND b.payment_status != 'CANCELLED'",
            showtime_id
        ),
    );
    
    SeatCode::find_by_statement(query)
        .all(db)
        .await
        .map(|rows| {
            rows.into_iter()
                .map(|r| r.seat_code)
                .collect()
        })
        .map_err(|e| BookingError::Database(e.to_string()))
}