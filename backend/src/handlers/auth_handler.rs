use axum::{extract::State, Json};
use sqlx::MySqlPool;
use crate::models::*;
use jsonwebtoken::{EncodingKey, Header};
use serde::Serialize;

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

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .map_err(|e| format!("JWT encode error: {}", e))
}

// Register new user fn
pub async fn register(
    State(pool): State<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
) -> Json<ApiResponse<UserInfo>> {
    if !payload.email.contains('@') {
        return Json(ApiResponse::error("Email tidak valid"));
    }

    if payload.password.len() < 6 {
        return Json(ApiResponse::error("Password minimal 6 karakter"));
    }

    let hashed_password = hash_password(&payload.password);
    let role = payload.role.unwrap_or_else(|| "customer".to_string()).to_uppercase();

    if role != "ADMIN" && role != "CUSTOMER" {
        return Json(ApiResponse::error("Role harus 'admin' atau 'customer'"));
    }

    let exists = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE email = ?"
    )
    .bind(&payload.email)
    .fetch_one(&pool)
    .await
    .unwrap_or(0);

    if exists > 0 {
        return Json(ApiResponse::error("Email sudah terdaftar"));
    }

    let insert_result = sqlx::query(
        "INSERT INTO users (name, email, password, role) VALUES (?, ?, ?, ?)"
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind(&role)
    .execute(&pool)
    .await;

    match insert_result {
        Ok(result) => {
            let user_id = result.last_insert_id() as i64;
            
            let user_info = UserInfo {
                id: user_id,
                name: payload.name,
                email: payload.email,
                role,
                cinema_id: None,
            };

            Json(ApiResponse::success("Registrasi berhasil", user_info))
        },
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Login user 
pub async fn login(
    State(pool): State<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    let hashed_password = hash_password(&payload.password);

    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password, role, CAST(created_at AS DATETIME) as created_at, NULL as updated_at FROM users WHERE email = ? AND password = ?"
    )
    .bind(&payload.email)
    .bind(&hashed_password)
    .fetch_optional(&pool)
    .await;

    match user_result {
        Ok(Some(user)) => {
            let cinema_id = if user.role.to_uppercase() == "ADMIN" {
                sqlx::query_scalar::<_, i64>(
                    "SELECT id FROM cinemas WHERE user_id = ? LIMIT 1"
                )
                .bind(user.id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None)
            } else {
                None
            };

            let user_info = UserInfo {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
                cinema_id,
            };

            let token = match generate_token(user.id) {
                Ok(t) => t,
                Err(e) => return Json(ApiResponse::error(&format!("Token generation error: {}", e))),
            };

            let login_response = LoginResponse {
                user: user_info,
                token,
            };

            Json(ApiResponse::success("Login berhasil", login_response))
        },
        Ok(None) => Json(ApiResponse::error("Email atau password salah")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Get user profile 
pub async fn get_profile(
    State(pool): State<MySqlPool>,
    user_id: i64,
) -> Json<ApiResponse<UserInfo>> {
    let user_result = sqlx::query_as::<_, User>(
        "SELECT id, name, email, password, role, CAST(created_at AS DATETIME) as created_at, CAST(updated_at AS DATETIME) as updated_at FROM users WHERE id = ?"
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await;

    match user_result {
        Ok(Some(user)) => {
            let cinema_id = if user.role.to_uppercase() == "ADMIN" {
                sqlx::query_scalar::<_, i64>(
                    "SELECT id FROM cinemas WHERE user_id = ? LIMIT 1"
                )
                .bind(user.id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(None)
            } else {
                None
            };

            let user_info = UserInfo {
                id: user.id,
                name: user.name,
                email: user.email,
                role: user.role,
                cinema_id,
            };

            Json(ApiResponse::success("Berhasil mengambil profile", user_info))
        },
        Ok(None) => Json(ApiResponse::error("User tidak ditemukan")),
        Err(e) => Json(ApiResponse::error(&format!("Database error: {}", e)))
    }
}

// Get cinemas by admin 
pub async fn get_admin_cinemas(
    State(pool): State<MySqlPool>,
    user_id: i64,
) -> Json<ApiResponse<Vec<crate::models::studio::Cinema>>> {
    sqlx::query_as::<_, crate::models::studio::Cinema>(
        "SELECT id, name, address, city, created_at, user_id FROM cinemas WHERE user_id = ?"
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await
    .map(|cinemas| Json(ApiResponse::success("Berhasil mengambil cinemas", cinemas)))
    .unwrap_or_else(|e| Json(ApiResponse::error(&format!("Database error: {}", e))))
}
