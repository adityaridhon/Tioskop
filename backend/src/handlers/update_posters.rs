use axum::{extract::State, Json};
use std::sync::Arc;
use crate::config::DatabasePools;
use crate::models::response::ApiResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct UpdateResult {
    pub updated_count: u64,
    pub message: String,
}

pub async fn update_movie_posters(
    State(pools): State<Arc<DatabasePools>>,
) -> Json<ApiResponse<UpdateResult>> {
    let pool = pools.get_central();
    // Update all movie posters and titles to match local assets
    // film-1: Sate Gagak, film-2: Pangku, film-3: Dopamin
    // film-4: Danyang Wingit, film-6: Now You See Me, film-7: Running Man, film-8: Keeper
    let updates = vec![
        (1, "/film-1.webp", "Sate Gagak"),
        (2, "/film-2.webp", "Pangku"),
        (3, "/film-3.webp", "Dopamin"),
        (4, "/film-4.webp", "Danyang Wingit"),
    ];
    
    let mut updated_count = 0u64;
    
    for (id, poster_url, title) in updates {
        match sqlx::query("UPDATE movies SET poster_url = ?, title = ? WHERE id = ?")
            .bind(poster_url)
            .bind(title)
            .bind(id)
            .execute(pool)
            .await
        {
            Ok(result) => {
                updated_count += result.rows_affected();
            }
            Err(e) => {
                eprintln!("Error updating movie {}: {}", id, e);
            }
        }
    }
    
    // Insert new movie if not exists
    let new_movies = vec![
        (5, "Now You See Me", "Thriller, Mystery", "8.7", 125, "Sekelompok pesulap melakukan perampokan sempurna", "/film-6.webp", "2024-01-15"),
        (6, "Wicked", "Fantasy, Musical", "9.0", 160, "Kisah sihir yang menakjubkan dari Oz", "/film-5.webp", "2024-11-22"),
        (7, "Running Man", "Action, Thriller", "8.5", 113, "Perlombaan mematikan untuk bertahan hidup", "/film-7.webp", "2024-06-10"),
        (8, "Keeper", "Drama, Sport", "8.3", 119, "Kisah inspiratif seorang penjaga gawang", "/film-8.webp", "2024-08-05"),
    ];
    
    for (id, title, genre, rating, duration, description, poster_url, release_date) in new_movies {
        let insert_result = sqlx::query(
            "INSERT INTO movies (id, title, genre, rating, duration, description, poster_url, release_date) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON DUPLICATE KEY UPDATE title = ?, poster_url = ?"
        )
        .bind(id)
        .bind(title)
        .bind(genre)
        .bind(rating)
        .bind(duration)
        .bind(description)
        .bind(poster_url)
        .bind(release_date)
        .bind(title)
        .bind(poster_url)
        .execute(pool)
        .await;
        
        if let Ok(result) = insert_result {
            updated_count += result.rows_affected();
        }
    }
    
    Json(ApiResponse::success(
        "Successfully updated movie posters",
        UpdateResult {
            updated_count,
            message: format!("Updated {} movie posters and titles", updated_count),
        },
    ))
}
