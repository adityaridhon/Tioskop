use crate::entities::{Studio, StudiosEntity};
use crate::models::studio::*;
use sea_orm::*;

#[derive(Debug)]
pub enum StudioError {
    NotFound(String),
    Database(String),
    Validation(String),
}

impl std::fmt::Display for StudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

type Result<T> = std::result::Result<T, StudioError>;

/// Validate studio name
fn validate_name(name: &str) -> Result<()> {
    if name.trim().is_empty() {
        Err(StudioError::Validation(
            "Nama studio tidak boleh kosong".to_string(),
        ))
    } else if name.len() > 100 {
        Err(StudioError::Validation(
            "Nama studio terlalu panjang (max 100)".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate capacity
fn validate_capacity(capacity: i32) -> Result<()> {
    if capacity <= 0 {
        Err(StudioError::Validation(
            "Kapasitas harus > 0".to_string(),
        ))
    } else if capacity > 500 {
        Err(StudioError::Validation(
            "Kapasitas terlalu besar (max 500)".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate cinema ID
fn validate_cinema_id(cinema_id: i64) -> Result<()> {
    if cinema_id <= 0 {
        Err(StudioError::Validation(
            "Cinema ID harus > 0".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Validate studio type
fn validate_type(studio_type: Option<&String>) -> Result<()> {
    if let Some(t) = studio_type {
        let valid_types = ["REGULAR", "IMAX", "4DX", "DOLBY", "VIP"];
        if !valid_types.contains(&t.to_uppercase().as_str()) {
            return Err(StudioError::Validation(format!(
                "Tipe studio tidak valid. Pilih: {}",
                valid_types.join(", ")
            )));
        }
    }
    Ok(())
}

/// Convert entity to model
fn entity_to_model(entity: Studio) -> crate::models::studio::Studio {
    crate::models::studio::Studio {
        id: entity.id,
        cinema_id: entity.cinema_id,
        name: entity.name,
        capacity: entity.capacity,
        r#type: entity.r#type,
    }
}

/// Convert entities to models
fn entities_to_models(entities: Vec<Studio>) -> Vec<crate::models::studio::Studio> {
    entities.into_iter().map(entity_to_model).collect()
}

/// Get all studios
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<crate::models::studio::Studio>> {
    StudiosEntity::find()
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Get studio by ID
pub async fn get_by_id(
    db: &DatabaseConnection,
    id: i64,
) -> Result<crate::models::studio::Studio> {
    StudiosEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))?
        .map(entity_to_model)
        .ok_or_else(|| StudioError::NotFound(format!("Studio dengan id {} tidak ditemukan", id)))
}

/// Get studios by cinema ID
pub async fn get_by_cinema(
    db: &DatabaseConnection,
    cinema_id: i64,
) -> Result<Vec<crate::models::studio::Studio>> {
    use crate::entities::studios::Column;

    StudiosEntity::find()
        .filter(Column::CinemaId.eq(cinema_id))
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Create new studio
pub async fn create(
    db: &DatabaseConnection,
    request: CreateStudioRequest,
) -> Result<crate::models::studio::Studio> {
    validate_cinema_id(request.cinema_id)?;
    validate_name(&request.name)?;
    validate_capacity(request.capacity)?;
    validate_type(request.r#type.as_ref())?;

    use crate::entities::studios::ActiveModel;

    let new_studio = ActiveModel {
        cinema_id: Set(Some(request.cinema_id)),
        name: Set(request.name),
        capacity: Set(request.capacity),
        r#type: Set(request.r#type),
        ..Default::default()
    };

    new_studio
        .insert(db)
        .await
        .map(entity_to_model)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Update studio
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    request: UpdateStudioRequest,
) -> Result<crate::models::studio::Studio> {
    if let Some(cinema_id) = request.cinema_id {
        validate_cinema_id(cinema_id)?;
    }

    if let Some(ref name) = request.name {
        validate_name(name)?;
    }

    if let Some(capacity) = request.capacity {
        validate_capacity(capacity)?;
    }

    validate_type(request.r#type.as_ref())?;

    let existing = StudiosEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))?
        .ok_or_else(|| StudioError::NotFound(format!("Studio dengan id {} tidak ditemukan", id)))?;

    use crate::entities::studios::ActiveModel;

    let updated = ActiveModel {
        id: Set(existing.id),
        cinema_id: Set(request.cinema_id.map(Some).unwrap_or(existing.cinema_id)),
        name: Set(request.name.unwrap_or(existing.name)),
        capacity: Set(request.capacity.unwrap_or(existing.capacity)),
        r#type: Set(request.r#type.or(existing.r#type)),
    };

    updated
        .update(db)
        .await
        .map(entity_to_model)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Delete studio
pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<i64> {
    let studio = StudiosEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))?
        .ok_or_else(|| StudioError::NotFound(format!("Studio dengan id {} tidak ditemukan", id)))?;

    use crate::entities::studios::ActiveModel;

    let active: ActiveModel = studio.into();

    active
        .delete(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))?;

    Ok(id)
}

/// Get studios by type
pub async fn get_by_type(
    db: &DatabaseConnection,
    studio_type: &str,
) -> Result<Vec<crate::models::studio::Studio>> {
    use crate::entities::studios::Column;

    StudiosEntity::find()
        .filter(Column::Type.eq(studio_type))
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Get studios with capacity 
pub async fn get_by_min_capacity(
    db: &DatabaseConnection,
    min_capacity: i32,
) -> Result<Vec<crate::models::studio::Studio>> {
    use crate::entities::studios::Column;

    StudiosEntity::find()
        .filter(Column::Capacity.gte(min_capacity))
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Count studios by cinema
pub async fn count_by_cinema(db: &DatabaseConnection, cinema_id: i64) -> Result<u64> {
    use crate::entities::studios::Column;

    StudiosEntity::find()
        .filter(Column::CinemaId.eq(cinema_id))
        .count(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Count total studios
pub async fn count_total(db: &DatabaseConnection) -> Result<u64> {
    StudiosEntity::find()
        .count(db)
        .await
        .map_err(|e| StudioError::Database(e.to_string()))
}

/// Search studios by name
pub async fn search(
    db: &DatabaseConnection,
    query: &str,
) -> Result<Vec<crate::models::studio::Studio>> {
    use crate::entities::studios::Column;

    let search_pattern = format!("%{}%", query);

    StudiosEntity::find()
        .filter(Column::Name.like(&search_pattern))
        .all(db)
        .await
        .map(entities_to_models)
        .map_err(|e| StudioError::Database(e.to_string()))
}