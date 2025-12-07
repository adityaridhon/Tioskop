use crate::entities::{Movie, MoviesEntity};
use crate::models::movie::*;
use sea_orm::*;

#[derive(Debug)]
pub enum MovieError {
    NotFound(String),
    Database(String),
    Validation(String),
}

impl std::fmt::Display for MovieError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{}", msg),
            Self::Database(msg) => write!(f, "Database error: {}", msg),
            Self::Validation(msg) => write!(f, "{}", msg),
        }
    }
}

type Result<T> = std::result::Result<T, MovieError>;


fn validate_title(title: &str) -> Result<()> {
    if title.trim().is_empty() {
        Err(MovieError::Validation("Title tidak boleh kosong".to_string()))
    } else if title.len() > 255 {
        Err(MovieError::Validation("Title terlalu panjang (max 255)".to_string()))
    } else {
        Ok(())
    }
}

fn validate_duration(duration: Option<i32>) -> Result<()> {
    if let Some(d) = duration {
        if d <= 0 {
            return Err(MovieError::Validation("Duration harus > 0".to_string()));
        }
        if d > 500 {
            return Err(MovieError::Validation("Duration terlalu panjang (max 500 menit)".to_string()));
        }
    }
    Ok(())
}

fn build_search_pattern(query: Option<String>) -> String {
    query
        .map(|q| format!("%{}%", q.trim()))
        .filter(|q| q.len() > 2) // Minimal 2 karakter setelah trim
        .unwrap_or_else(|| "%".to_string())
}

/// Get all movies
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Movie>> {
    MoviesEntity::find()
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Get movie by ID
pub async fn get_by_id(db: &DatabaseConnection, id: i64) -> Result<Movie> {
    MoviesEntity::find_by_id(id)
        .one(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))?
        .ok_or_else(|| MovieError::NotFound(format!("Film dengan id {} tidak ditemukan", id)))
}

/// Search movies by title
pub async fn search(db: &DatabaseConnection, query: Option<String>) -> Result<Vec<Movie>> {
    use crate::entities::movies::Column;
    
    let search_pattern = build_search_pattern(query);
    
    MoviesEntity::find()
        .filter(Column::Title.like(&search_pattern))
        .order_by_desc(Column::ReleaseDate)
        .limit(15)
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Get movies by genre
pub async fn get_by_genre(db: &DatabaseConnection, genre: &str) -> Result<Vec<Movie>> {
    use crate::entities::movies::Column;
    
    MoviesEntity::find()
        .filter(Column::Genre.eq(genre))
        .order_by_desc(Column::ReleaseDate)
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Get movies by rating
pub async fn get_by_rating(db: &DatabaseConnection, rating: &str) -> Result<Vec<Movie>> {
    use crate::entities::movies::Column;
    
    MoviesEntity::find()
        .filter(Column::Rating.eq(rating))
        .order_by_desc(Column::ReleaseDate)
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}


/// Create new movie - Functional approach
pub async fn create(db: &DatabaseConnection, request: CreateMovieRequest) -> Result<Movie> {
    validate_title(&request.title)?;
    validate_duration(request.duration)?;
    
    use crate::entities::movies::ActiveModel;
    
    let new_movie = ActiveModel {
        title: Set(request.title),
        genre: Set(request.genre),
        rating: Set(request.rating),
        duration: Set(request.duration),
        description: Set(request.description),
        poster_url: Set(request.poster_url),
        release_date: Set(request.release_date),
        ..Default::default()
    };
    
    new_movie
        .insert(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Update movie
pub async fn update(
    db: &DatabaseConnection,
    id: i64,
    request: UpdateMovieRequest,
) -> Result<Movie> {
    if let Some(ref title) = request.title {
        validate_title(title)?;
    }
    
    validate_duration(request.duration)?;
    
    let existing = get_by_id(db, id).await?;
    
    use crate::entities::movies::ActiveModel;
    
    let updated = ActiveModel {
        id: Set(existing.id),
        title: Set(request.title.unwrap_or(existing.title)),
        genre: Set(request.genre.or(existing.genre)),
        rating: Set(request.rating.or(existing.rating)),
        duration: Set(request.duration.or(existing.duration)),
        description: Set(request.description.or(existing.description)),
        poster_url: Set(request.poster_url.or(existing.poster_url)),
        release_date: Set(request.release_date.or(existing.release_date)),
    };
    
    updated
        .update(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Delete movie
pub async fn delete(db: &DatabaseConnection, id: i64) -> Result<i64> {
    let movie = get_by_id(db, id).await?;
    
    use crate::entities::movies::ActiveModel;
    
    let active: ActiveModel = movie.into();
    
    active
        .delete(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))?;
    
    Ok(id)
}

/// Get movies with pagination 
pub async fn get_paginated(
    db: &DatabaseConnection,
    page: u64,
    per_page: u64,
) -> Result<Vec<Movie>> {
    use crate::entities::movies::Column;
    
    let offset = (page - 1) * per_page;
    
    MoviesEntity::find()
        .order_by_desc(Column::ReleaseDate)
        .limit(per_page)
        .offset(offset)
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Count total movies 
pub async fn count_total(db: &DatabaseConnection) -> Result<u64> {
    MoviesEntity::find()
        .count(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}

/// Get latest movies 
pub async fn get_latest(db: &DatabaseConnection, limit: u64) -> Result<Vec<Movie>> {
    use crate::entities::movies::Column;
    
    MoviesEntity::find()
        .order_by_desc(Column::ReleaseDate)
        .limit(limit)
        .all(db)
        .await
        .map_err(|e| MovieError::Database(e.to_string()))
}