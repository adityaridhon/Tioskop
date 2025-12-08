use crate::entities::{Showtime, ShowtimesEntity};
use crate::models::showtime::*;
use sea_orm::*;
use chrono::{NaiveDateTime, TimeZone};
use rust_decimal::Decimal;

#[derive(Debug)]
pub enum ShowtimeError {
    NotFound(String),
    Database(String),
    Validation(String),
}

impl std::fmt::Display for ShowtimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

type Result<T> = std::result::Result<T, ShowtimeError>;

/// Validate movie ID
fn validate_movie_id(movie_id: i64) -> Result<()> {
    if movie_id <= 0 {
        Err(ShowtimeError::Validation(
            "Movie ID harus > 0".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate studio ID
fn validate_studio_id(studio_id: i64) -> Result<()> {
    if studio_id <= 0 {
        Err(ShowtimeError::Validation(
            "Studio ID harus > 0".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate price
fn validate_price(price: Decimal) -> Result<()> {
    if price <= Decimal::ZERO {
        Err(ShowtimeError::Validation(
            "Price harus > 0".to_string(),
        ))
    } else if price > Decimal::from(1000000) {
        Err(ShowtimeError::Validation(
            "Price terlalu tinggi (max 1,000,000)".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate start time is in future
fn validate_start_time(start_time: NaiveDateTime) -> Result<()> {
    let now = chrono::Local::now().naive_local();
    if start_time < now {
        Err(ShowtimeError::Validation(
            "Start time harus di masa depan".to_string(),
        ))
    } else {
        Ok(())
    }
}

// ===== HELPER FUNCTIONS (Pure Functions) =====

/// Convert entity model to API model
fn entity_to_model(entity: Showtime) -> crate::models::showtime::Showtime {
    crate::models::showtime::Showtime {
        id: entity.id,
        movie_id: entity.movie_id,
        studio_id: entity.studio_id,
        start_time: entity.start_time.map(|dt| dt.naive_local()),
        price: entity.price,
    }
}

/// Convert entities to models
fn entities_to_models(
    entities: Vec<Showtime>,
) -> Vec<crate::models::showtime::Showtime> {
    entities.into_iter().map(entity_to_model).collect()
}

/// Build datetime with timezone
fn build_datetime_with_tz(
    naive: NaiveDateTime,
) -> Result<chrono::DateTime<chrono::Local>> {
    use chrono::Local;

    Local
        .from_local_datetime(&naive)
        .single()
        .ok_or_else(|| ShowtimeError::Validation("Invalid datetime".to_string()))
}

/// Get all showtimes
pub async fn get_all(
    db: &DatabaseConnection,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    ShowtimesEntity::find()
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Get showtime by ID
pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> Result<crate::models::showtime::Showtime> {
    ShowtimesEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ShowtimeError::Database(e.to_string()))?
        .map(entity_to_model)
        .ok_or_else(|| {
            ShowtimeError::NotFound(format!("Showtime dengan id {} tidak ditemukan", id))
        })
}

/// Get showtimes by movie ID
pub async fn get_by_movie(
    db: &DatabaseConnection,
    movie_id: i64,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    ShowtimesEntity::find()
        .filter(Column::MovieId.eq(movie_id))
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Get showtimes by studio ID
pub async fn get_by_studio(
    db: &DatabaseConnection,
    studio_id: i64,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    ShowtimesEntity::find()
        .filter(Column::StudioId.eq(studio_id))
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

pub async fn create(
    db: &DatabaseConnection,
    request: CreateShowtimeRequest,
) -> Result<crate::models::showtime::Showtime> {
    validate_movie_id(request.movie_id)?;
    validate_studio_id(request.studio_id)?;
    validate_price(request.price)?;
    validate_start_time(request.start_time)?;

    use crate::entities::showtimes::ActiveModel;

    let start_time_with_tz = build_datetime_with_tz(request.start_time)?;

    let new_showtime = ActiveModel {
        movie_id: Set(Some(request.movie_id)),
        studio_id: Set(Some(request.studio_id)),
        start_time: Set(Some(start_time_with_tz)),
        price: Set(Some(request.price)),
        ..Default::default()
    };

    new_showtime
        .insert(db)
        .await
        .map(entity_to_model)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Update showtime
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    request: UpdateShowtimeRequest,
) -> Result<crate::models::showtime::Showtime> {
    if let Some(movie_id) = request.movie_id {
        validate_movie_id(movie_id)?;
    }

    if let Some(studio_id) = request.studio_id {
        validate_studio_id(studio_id)?;
    }

    if let Some(price) = request.price {
        validate_price(price)?;
    }

    if let Some(start_time) = request.start_time {
        validate_start_time(start_time)?;
    }

    let existing = ShowtimesEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ShowtimeError::Database(e.to_string()))?
        .ok_or_else(|| {
            ShowtimeError::NotFound(format!("Showtime dengan id {} tidak ditemukan", id))
        })?;

    use crate::entities::showtimes::ActiveModel;

    let updated_movie_id = request.movie_id.map(Some).unwrap_or(existing.movie_id);
    let updated_studio_id = request.studio_id.map(Some).unwrap_or(existing.studio_id);
    let updated_price = request.price.map(Some).unwrap_or(existing.price);

    let updated_start_time = if let Some(new_time) = request.start_time {
        Some(build_datetime_with_tz(new_time)?)
    } else {
        existing.start_time
    };

    let updated = ActiveModel {
        id: Set(existing.id),
        movie_id: Set(updated_movie_id),
        studio_id: Set(updated_studio_id),
        start_time: Set(updated_start_time),
        price: Set(updated_price),
    };

    updated
        .update(db)
        .await
        .map(entity_to_model)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Delete showtime
pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<i64> {
    let showtime = ShowtimesEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| ShowtimeError::Database(e.to_string()))?
        .ok_or_else(|| {
            ShowtimeError::NotFound(format!("Showtime dengan id {} tidak ditemukan", id))
        })?;

    use crate::entities::showtimes::ActiveModel;

    let active: ActiveModel = showtime.into();

    active
        .delete(db)
        .await
        .map_err(|e| ShowtimeError::Database(e.to_string()))?;

    Ok(id)
}

/// Get upcoming showtimes
pub async fn get_upcoming(
    db: &DatabaseConnection,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;
    use chrono::Local;

    let now = Local::now();

    ShowtimesEntity::find()
        .filter(Column::StartTime.gte(now))
        .order_by_asc(Column::StartTime)
        .limit(20)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Get showtimes with filters 
pub async fn get_filtered(
    db: &DatabaseConnection,
    movie_id: Option<i64>,
    studio_id: Option<i64>,
    from_date: Option<NaiveDateTime>,
    to_date: Option<NaiveDateTime>,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    let mut query = ShowtimesEntity::find();

    // Apply filters functionally
    if let Some(mid) = movie_id {
        query = query.filter(Column::MovieId.eq(mid));
    }

    if let Some(sid) = studio_id {
        query = query.filter(Column::StudioId.eq(sid));
    }

    if let Some(from) = from_date {
        let from_tz = build_datetime_with_tz(from)?;
        query = query.filter(Column::StartTime.gte(from_tz));
    }

    if let Some(to) = to_date {
        let to_tz = build_datetime_with_tz(to)?;
        query = query.filter(Column::StartTime.lte(to_tz));
    }

    query
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Get showtimes by date range
pub async fn get_by_date_range(
    db: &DatabaseConnection,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    let start_tz = build_datetime_with_tz(start)?;
    let end_tz = build_datetime_with_tz(end)?;

    ShowtimesEntity::find()
        .filter(Column::StartTime.between(start_tz, end_tz))
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}

/// Get today's showtimes
pub async fn get_today(
    db: &DatabaseConnection,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use chrono::Local;

    let today = Local::now().date_naive();
    let start = today.and_hms_opt(0, 0, 0).unwrap();
    let end = today.and_hms_opt(23, 59, 59).unwrap();

    get_by_date_range(db, start, end).await
}

/// Get showtimes by movie and date
pub async fn get_by_movie_and_date(
    db: &DatabaseConnection,
    movie_id: i64,
    date: chrono::NaiveDate,
) -> Result<Vec<crate::models::showtime::Showtime>> {
    use crate::entities::showtimes::Column;

    let start = date.and_hms_opt(0, 0, 0).unwrap();
    let end = date.and_hms_opt(23, 59, 59).unwrap();
    let start_tz = build_datetime_with_tz(start)?;
    let end_tz = build_datetime_with_tz(end)?;

    ShowtimesEntity::find()
        .filter(Column::MovieId.eq(movie_id))
        .filter(Column::StartTime.between(start_tz, end_tz))
        .order_by_asc(Column::StartTime)
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| ShowtimeError::Database(e.to_string()))
}