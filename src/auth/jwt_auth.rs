use axum::{
    extract::Request,
    response::Response,
    http::{HeaderMap, StatusCode},
    middleware::Next
};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{
    types::response_types::ApiResponse, utils::env_utils::load_env_vars
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub exp: usize,
    pub is_admin: bool,
}

pub fn validate_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let config = load_env_vars().unwrap();

    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    decode::<Claims>(token, &DecodingKey::from_secret(config.secret_key.as_bytes()), &validation)
        .map(|data| data.claims)
}

pub async fn jwt_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiResponse> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiResponse::err(StatusCode::UNAUTHORIZED))?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiResponse::err(StatusCode::UNAUTHORIZED))?;

    let claims = validate_token(token).map_err(|_| ApiResponse::err(StatusCode::UNAUTHORIZED))?;

    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

pub fn generate_jwt(user_id: i64, is_admin: bool) -> String {
    let config = load_env_vars().unwrap();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        is_admin,
    };

    let secret = config.secret_key;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}