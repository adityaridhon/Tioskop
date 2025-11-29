use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::extract::FromRequestParts;
use serde::Deserialize;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Deserialize)]
struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct AuthUser(pub i64);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // Get Authorization header
            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|v| v.to_str().ok())
                .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

            if !auth_header.starts_with("Bearer ") {
                return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header"));
            }

            let token = &auth_header[7..];

            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "tioskop_dev_secret".to_string());

            let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default())
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

            let user_id = token_data.claims.sub.parse::<i64>().map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token sub"))?;

            Ok(AuthUser(user_id))
        }
    }
}
