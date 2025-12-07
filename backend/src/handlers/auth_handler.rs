use crate::middleware::auth::AuthUser;
use crate::models::*;
use crate::entities::UsersEntity;
use axum::{Json, extract::State};
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, ColumnTrait, QueryFilter, PaginatorTrait};

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// Simple password hashing
fn hash_password(password: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    password.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

// Simple token generation
fn generate_token(user_id: i64) -> Result<String, String> {
    // JWT with simple HMAC secret from env (fallback for dev)
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "tioskop_dev_secret".to_string());

    // token expiry: 24 hours from now
    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| format!("JWT encode error: {}", e))
}

// Register new user fn
pub async fn register(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<RegisterRequest>,
) -> Json<ApiResponse<UserInfo>> {
    use crate::entities::users::Column;

    if !payload.email.contains('@') {
        return Json(ApiResponse::error("Email tidak valid"));
    }

    if payload.password.len() < 6 {
        return Json(ApiResponse::error("Password minimal 6 karakter"));
    }

    let hashed_password = hash_password(&payload.password);
    let role = payload
        .role
        .unwrap_or_else(|| "customer".to_string())
        .to_uppercase();

    if role != "ADMIN" && role != "CUSTOMER" {
        return Json(ApiResponse::error("Role harus 'admin' atau 'customer'"));
    }

    // Check if email exists
    let exists = UsersEntity::find()
        .filter(Column::Email.eq(&payload.email))
        .count(&db)
        .await
        .unwrap_or(0);

    if exists > 0 {
        return Json(ApiResponse::error("Email sudah terdaftar"));
    }

    // Create new user
    use crate::entities::users::ActiveModel;
    let new_user = ActiveModel {
        name: Set(payload.name.clone()),
        email: Set(payload.email.clone()),
        password: Set(hashed_password),
        role: Set(role.clone()),
        ..Default::default()
    };

    match new_user.insert(&db).await {
        Ok(user) => {
            let user_info = UserInfo {
                id: user.id,
                name: payload.name,
                email: payload.email,
                role,
                cinema_id: None,
            };

            Json(ApiResponse::success("Registrasi berhasil", user_info))
        }
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Login user
pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(payload): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    use crate::entities::users::Column;
    use sea_orm::QuerySelect;

    let hashed_password = hash_password(&payload.password);

    // Select only needed columns, skip timestamp columns
    let user_result = UsersEntity::find()
        .select_only()
        .column(Column::Id)
        .column(Column::Name)
        .column(Column::Email)
        .column(Column::Role)
        .filter(Column::Email.eq(&payload.email))
        .filter(Column::Password.eq(&hashed_password))
        .into_tuple::<(i64, String, String, String)>()
        .one(&db)
        .await;

    match user_result {
        Ok(Some((id, name, email, role))) => {
            // TODO: Implement cinema lookup when cinemas entity is created
            let cinema_id = None;

            let user_info = UserInfo {
                id,
                name,
                email,
                role,
                cinema_id,
            };

            let token = match generate_token(id) {
                Ok(t) => t,
                Err(e) => {
                    return Json(ApiResponse::error(&format!(
                        "Token generation error: {}",
                        e
                    )));
                }
            };

            let login_response = LoginResponse {
                user: user_info,
                token,
            };

            Json(ApiResponse::success("Login berhasil", login_response))
        }
        Ok(None) => Json(ApiResponse::error("Email atau password salah")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get user profile
pub async fn get_profile(
    State(db): State<DatabaseConnection>,
    AuthUser(user_id): AuthUser,
) -> Json<ApiResponse<UserInfo>> {
    use crate::entities::users::Column;
    use sea_orm::QuerySelect;

    // Select only needed columns, skip timestamp columns
    let user_result = UsersEntity::find_by_id(user_id)
        .select_only()
        .column(Column::Id)
        .column(Column::Name)
        .column(Column::Email)
        .column(Column::Role)
        .into_tuple::<(i64, String, String, String)>()
        .one(&db)
        .await;

    match user_result {
        Ok(Some((id, name, email, role))) => {
            // TODO: Implement cinema lookup when cinemas entity is created
            let cinema_id = None;

            let user_info = UserInfo {
                id,
                name,
                email,
                role,
                cinema_id,
            };

            Json(ApiResponse::success(
                "Berhasil mengambil profile",
                user_info,
            ))
        }
        Ok(None) => Json(ApiResponse::error("User tidak ditemukan")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e))),
    }
}

// Get cinemas by admin
pub async fn get_admin_cinemas(
    State(db): State<DatabaseConnection>,
    user_id: i64,
) -> Json<ApiResponse<Vec<crate::models::studio::Cinema>>> {
    // TODO: Implement when Cinemas entity is created
    // For now, return empty array
    Json(ApiResponse::success("Berhasil mengambil cinemas", vec![]))
}
