use axum::{extract::Request, http::{HeaderMap, StatusCode}, middleware::Next, response::Response};
use dotenv::{dotenv, var};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64, // Subject (e.g., user ID)
    pub exp: usize,  // Expiration time
}

pub struct Config {
    pub dbuser: String,
    pub dbname: String,
    pub dbpwd: String,
    pub secret_key: String,
}

pub fn load_env_vars() -> Result<Config, Box<dyn std::error::Error>> {
    dotenv()?;

    Ok(Config {
        dbname: var("POSTGRES_DB")?,
        dbuser: var("POSTGRES_USER")?,
        dbpwd: var("POSTGRES_PASSWORD")?,
        secret_key: var("SECRET_KEY")?,
        cors_allowed: var("CORS_ALLOWED")
    })
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
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = validate_token(token).map_err(|_| StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
