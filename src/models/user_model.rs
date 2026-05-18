use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Row};

#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub login: String,
    pub pw_hash: String,
    pub cell_number: Option<String>,
    pub is_activated: bool,
    pub created_at: String,
    pub updated_at: String,
    pub is_admin: bool,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub login: String,
    pub password: String,
    pub cell_number: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub login: Option<String>,
    pub password: Option<String>,
    pub cell_number: Option<String>,
    pub is_activated: Option<bool>,
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
            is_activated: row.get("is_activated"),
            is_admin: row.get("is_admin"),
            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),
            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}
