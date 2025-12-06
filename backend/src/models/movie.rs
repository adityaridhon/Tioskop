use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow, Clone)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
}

#[derive(Deserialize)]
pub struct CreateMovieRequest {
    pub title: String,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
}

#[derive(Deserialize)]
pub struct UpdateMovieRequest {
    pub title: Option<String>,
    pub genre: Option<String>,
    pub rating: Option<String>,
    pub duration: Option<i32>,
    pub description: Option<String>,
    pub poster_url: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
}

#[derive(Deserialize)]
pub struct SearchParams {
    pub q: Option<String>,
}
