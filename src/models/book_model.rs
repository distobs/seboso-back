use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Row};

#[derive(Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub published_at: Option<String>,
    pub cover_url: Option<String>,
    pub cover_type: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub isbn_10_code: Option<String>,
    pub isbn_13_code: Option<String>,
    pub publisher: Option<String>,
    pub pages: Option<i64>,
    pub dimensions: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateBook {
    pub title: String,
    pub author: String,
    pub description: Option<String>,
    pub published_at: Option<String>,
    pub cover_url: Option<String>,
    pub cover_type: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub isbn_10_code: Option<String>,
    pub isbn_13_code: Option<String>,
    pub publisher: Option<String>,
    pub pages: Option<i64>,
    pub dimensions: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub description: Option<String>,
    pub published_at: Option<String>,
    pub cover_url: Option<String>,
    pub cover_type: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub isbn_10_code: Option<String>,
    pub isbn_13_code: Option<String>,
    pub publisher: Option<String>,
    pub pages: Option<i64>,
    pub dimensions: Option<String>,
}

impl From<&Row> for Book {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            author: row.get("author"),
            description: row.get("description"),
            published_at: row.get("published_at"),
            cover_url: row.get("cover_url"),
            cover_type: row.get("cover_type"),
            edition: row.get("edition"),
            language: row.get("language"),
            genre: row.get("genre"),
            isbn_10_code: row.get("isbn_10_code"),
            isbn_13_code: row.get("isbn_13_code"),
            publisher: row.get("publisher"),
            pages: row.get("pages"),
            dimensions: row.get("dimensions"),

            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),

            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}
