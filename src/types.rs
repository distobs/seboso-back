use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::{DateTime, Utc};
use tokio_postgres::NoTls;
use tokio_postgres::{Row};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

// Entity models
#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub login: String,
    pub pw_hash: String,
    pub cell_number: String,
    pub role: i64,
    pub is_activated: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub login: String,
    pub password: String,
    pub cell_number: String,
    pub role: i64,
    pub is_activated: i64
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub login: String,
    pub password: String,
}

impl From<&Row> for User {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            login: row.get("login"),
            pw_hash: row.get("password"),
            cell_number: row.get("cell_number"),
            role: row.get("role"),
            is_activated: row.get("is_activated"),
            created_at: row.get::<&str, DateTime<Utc>>("created_at")
                .to_string(),
            updated_at: row.get::<&str, DateTime<Utc>>("updated_at")
                .to_string(),
        }
    }
}

// DB connection
pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;