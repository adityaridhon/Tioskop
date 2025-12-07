use crate::models::*;
use crate::entities::{Seat, SeatsEntity, StudiosEntity};
use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, QueryOrder, FromQueryResult};

// Get all seats
pub async fn get_all_seats(State(db): State<DatabaseConnection>) -> Json<ApiResponse<Vec<Seat>>> {
    SeatsEntity::find()
        .all(&db)
        .await
        .map(|seats| {
            Json(ApiResponse::success(
                "Berhasil mengambil semua seats",
                seats,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get seats by studio_id
pub async fn get_seats_by_studio(
    State(db): State<DatabaseConnection>,
    Path(studio_id): Path<i64>,
) -> Json<ApiResponse<Vec<Seat>>> {
    use crate::entities::seats::Column;

    SeatsEntity::find()
        .filter(Column::StudioId.eq(studio_id))
        .order_by_asc(Column::SeatRow)
        .order_by_asc(Column::SeatCol)
        .all(&db)
        .await
        .map(|seats| Json(ApiResponse::success("Berhasil mengambil seats untuk studio ini", seats)))
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get seats by showtime
// TODO: Complex JOIN query - consider using sea_query for advanced queries
pub async fn get_seats_by_showtime(
    State(db): State<DatabaseConnection>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<SeatWithBookingStatus>>> {
    use sea_orm::FromQueryResult;

    // Using raw SQL for complex JOIN (SeaORM doesn't handle this well)
    let sql = r#"
        SELECT 
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
        ORDER BY s.seat_row, s.seat_col
    "#;

    #[derive(Debug, FromQueryResult)]
    struct SeatQueryResult {
        id: i64,
        studio_id: Option<i64>,
        seat_code: String,
        seat_row: Option<i32>,
        seat_col: Option<i32>,
        seat_status: Option<String>,
        booking_id: Option<i64>,
    }

    let statement = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::MySql,
        sql,
        vec![showtime_id.into()],
    );

    SeatQueryResult::find_by_statement(statement)
        .all(&db)
        .await
        .map(|rows| {
            let seats = rows
                .into_iter()
                .map(|row| SeatWithBookingStatus {
                    id: row.id,
                    studio_id: row.studio_id.unwrap_or(0),
                    seat_code: row.seat_code,
                    seat_row: row.seat_row,
                    seat_col: row.seat_col,
                    seat_status: row.seat_status,
                    is_booked: row.booking_id.is_some(),
                    booking_id: row.booking_id,
                })
                .collect::<Vec<_>>();
            Json(ApiResponse::success(
                "Berhasil mengambil seats dengan status booking",
                seats,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Get available seats only for a showtime
pub async fn get_available_seats_by_showtime(
    State(db): State<DatabaseConnection>,
    Path(showtime_id): Path<i64>,
) -> Json<ApiResponse<Vec<Seat>>> {
    use sea_orm::FromQueryResult;

    let sql = r#"
        SELECT DISTINCT
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
        ORDER BY s.seat_row, s.seat_col
    "#;

    let statement = sea_orm::Statement::from_sql_and_values(
        sea_orm::DatabaseBackend::MySql,
        sql,
        vec![showtime_id.into()],
    );

    Seat::find_by_statement(statement)
        .all(&db)
        .await
        .map(|seats| {
            Json(ApiResponse::success(
                "Berhasil mengambil seats yang tersedia",
                seats,
            ))
        })
        .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}

// Generate seats studio
pub async fn generate_seats_for_studio(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<GenerateSeatsRequest>,
) -> Json<ApiResponse<GenerateSeatsResponse>> {
    // Check if studio exists
    let studio_exists = StudiosEntity::find_by_id(payload.studio_id)
        .one(&db)
        .await;

    match studio_exists {
        Ok(Some(_)) => {
            use crate::entities::seats::ActiveModel;

            // Generate seat codes (immutable)
            let seat_codes: Vec<(String, i32, i32)> = (1..=payload.rows)
                .flat_map(|row| {
                    let row_letter = char::from(b'A' + (row - 1) as u8).to_string();
                    (1..=payload.seats_per_row)
                        .map(move |col| (format!("{}{}", row_letter, col), row, col))
                        .collect::<Vec<_>>()
                })
                .collect();

            // ✅ IMMUTABLE APPROACH: Insert all seats sequentially
            let insert_count = {
                let mut count = 0i32;
                for (seat_code, row, col) in seat_codes {
                    let new_seat = ActiveModel {
                        studio_id: Set(Some(payload.studio_id)),
                        seat_code: Set(seat_code.clone()),
                        seat_row: Set(Some(row)),
                        seat_col: Set(Some(col)),
                        seat_status: Set(Some("AVAILABLE".to_string())),
                        ..Default::default()
                    };
                    if new_seat.insert(&db).await.is_ok() {
                        count += 1;
                    }
                }
                count
            };

            Json(ApiResponse::success(
                &format!(
                    "Berhasil generate {} kursi untuk studio {}",
                    insert_count, payload.studio_id
                ),
                GenerateSeatsResponse {
                    studio_id: payload.studio_id,
                    total_seats_created: insert_count,
                },
            ))
        }
        Ok(None) => Json(ApiResponse::error(&format!(
            "Studio dengan id {} tidak ditemukan",
            payload.studio_id
        ))),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}
