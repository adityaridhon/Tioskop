use crate::models::response::ApiResponse;
use crate::entities::MoviesEntity;
use axum::{Json, extract::State};
use serde::Serialize;
use sea_orm::{DatabaseConnection, Set, EntityTrait, ActiveModelTrait, IntoActiveModel};
use chrono::NaiveDate;

#[derive(Serialize)]
pub struct UpdateResult {
    pub updated_count: u64,
    pub message: String,
}

pub async fn update_movie_posters(
    State(db): State<DatabaseConnection>,
) -> Json<ApiResponse<UpdateResult>> {
    // Update existing movie posters and titles - SeaORM approach
    let updates = vec![
        (1, "/film-1.webp", "Sate Gagak"),
        (2, "/film-2.webp", "Pangku"),
        (3, "/film-3.webp", "Dopamin"),
        (4, "/film-4.webp", "Danyang Wingit"),
    ];

    let update_count = {
        let mut count = 0u64;
        for (id, poster_url, title) in updates {
            if let Ok(Some(movie)) = MoviesEntity::find_by_id(id).one(&db).await {
                let mut active_movie = movie.into_active_model();
                active_movie.poster_url = Set(Some(poster_url.to_string()));
                active_movie.title = Set(title.to_string());
                
                if active_movie.update(&db).await.is_ok() {
                    count += 1;
                }
            }
        }
        count
    };

    // Insert/update new movies with SeaORM
    use crate::entities::movies::ActiveModel;
    
    let new_movies_data = vec![
        (5, "Now You See Me", "Thriller, Mystery", "8.7", 125, 
         "Sekelompok pesulap melakukan perampokan sempurna", "/film-6.webp", "2024-01-15"),
        (6, "Wicked", "Fantasy, Musical", "9.0", 160, 
         "Kisah sihir yang menakjubkan dari Oz", "/film-5.webp", "2024-11-22"),
        (7, "Running Man", "Action, Thriller", "8.5", 113, 
         "Perlombaan mematikan untuk bertahan hidup", "/film-7.webp", "2024-06-10"),
        (8, "Keeper", "Drama, Sport", "8.3", 119, 
         "Kisah inspiratif seorang penjaga gawang", "/film-8.webp", "2024-08-05"),
    ];

    let insert_count = {
        let mut count = 0u64;
        for (id, title, genre, rating, duration, description, poster_url, release_date) in new_movies_data {
            match MoviesEntity::find_by_id(id).one(&db).await {
                Ok(Some(existing)) => {
                    // Update existing
                    let mut active_movie = existing.into_active_model();
                    active_movie.title = Set(title.to_string());
                    active_movie.poster_url = Set(Some(poster_url.to_string()));
                    if active_movie.update(&db).await.is_ok() {
                        count += 1;
                    }
                }
                Ok(None) => {
                    // Insert new
                    let new_movie = ActiveModel {
                        id: Set(id),
                        title: Set(title.to_string()),
                        genre: Set(Some(genre.to_string())),
                        rating: Set(Some(rating.to_string())),
                        duration: Set(Some(duration)),
                        description: Set(Some(description.to_string())),
                        poster_url: Set(Some(poster_url.to_string())),
                        release_date: Set(NaiveDate::parse_from_str(release_date, "%Y-%m-%d").ok()),
                    };
                    if new_movie.insert(&db).await.is_ok() {
                        count += 1;
                    }
                }
                Err(_) => {}
            }
        }
        count
    };

    let total_updated = update_count + insert_count;

    Json(ApiResponse::success(
        "Successfully updated movie posters",
        UpdateResult {
            updated_count: total_updated,
            message: format!("Updated {} movie posters and titles", total_updated),
        },
    ))
}
