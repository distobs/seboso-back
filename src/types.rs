use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio_postgres::{NoTls, Row};

// Pagination
#[derive(Deserialize)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

// Responses and errors
#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    DB(tokio_postgres::Error),
    BadReq(String),
}

impl From<tokio_postgres::Error> for ApiError {
    fn from(err: tokio_postgres::Error) -> Self {
        ApiError::DB(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::DB(err) => {
                eprintln!("Database error: {:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "success": false,
                        "message": "Database error",
                    })),
                )
                    .into_response()
            }

            ApiError::BadReq(msg) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": msg,
                })),
            )
                .into_response(),
        }
    }
}

// Entity models

/** USER **/
#[derive(Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub login: String,
    pub pw_hash: String,
    pub cell_number: String,
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
    pub is_activated: i64,
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
            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),
            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}

/** STORE **/
#[derive(Serialize)]
pub struct Store {
    pub id: i64,
    pub name: String,
    pub cnpj: String,
    pub street: String,
    pub number: i64,
    pub city: String,
    pub state: String,
    pub city_block: String,
    pub cep: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct CreateStore {
    pub name: String,
    pub cnpj: String,
    pub street: String,
    pub number: i64,
    pub city: String,
    pub state: String,
    pub city_block: String,
    pub cep: String,
    pub workers: Vec<CreateUserStore>,
}

impl From<&Row> for Store {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            cnpj: row.get("cnpj"),
            street: row.get("street"),
            number: row.get("number"),
            city: row.get("city"),
            state: row.get("state"),
            city_block: row.get("city_block"),
            cep: row.get("cep"),
            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),
            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}

/** USER_STORE **/
#[derive(Deserialize)]
pub struct UserStore {
    pub store_id: i64,
    pub user_id: i64,
    pub role: String,
}

#[derive(Deserialize)]
pub struct CreateUserStore {
    pub user_id: i64,
    pub role: String,
}

impl From<&Row> for UserStore {
    fn from(row: &Row) -> Self {
        Self {
            store_id: row.get("store_id"),
            user_id: row.get("user_id"),
            role: row.get("role"),
        }
    }
}

impl UserStore {
    pub async fn from_user_id(pool: &DbPool, user_id: i64) -> Result<Vec<UserStore>, ApiError> {
        let conn = pool.get().await.unwrap();

        let rows = conn
            .query("SELECT * FROM user_store WHERE user_id = $1", &[&user_id])
            .await?;

        Ok(rows.iter().map(UserStore::from).collect())
    }

    pub async fn from_store_id(pool: &DbPool, store_id: i64) -> Result<Vec<UserStore>, ApiError> {
        let conn = pool.get().await.unwrap();

        let rows = conn
            .query("SELECT * FROM user_store WHERE store_id = $1", &[&store_id])
            .await?;

        Ok(rows.iter().map(UserStore::from).collect())
    }

    pub async fn check_role_in_store(
        user_id: i64,
        store_id: i64,
        roles: &[&str],
        pool: &DbPool,
    ) -> Result<bool, ApiError> {
        let conn = pool.get().await.unwrap();

        let row = conn
            .query_opt(
                "
                SELECT 1
                FROM user_store
                WHERE user_id = $1
                AND store_id = $2
                AND role = ANY($3)
                LIMIT 1
            ",
                &[&user_id, &store_id, &roles],
            )
            .await?;

        Ok(row.is_some())
    }
}

/** BOOK **/
#[derive(Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub launched_at: String,
    pub cover_type: String,
    pub author: String,
    pub edition: String,
    pub language: String,
    pub genre: String,
    pub isbn_10_code: i64,
    pub isbn_13_code: Option<String>,
    pub publisher: String,
    pub pages: i64,
    pub dimentions: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct BookQuery {
    pub isbn_10: i64,
}

#[derive(Deserialize)]
pub struct CreateBookDto {
    pub title: String,
    pub description: String,
    pub launched_at: String,
    pub cover_type: String,
    pub author: String,
    pub edition: String,
    pub language: String,
    pub genre: String,
    pub isbn_10_code: i64,
    pub isbn_13_code: Option<String>,
    pub publisher: String,
    pub pages: i64,
    pub dimentions: String,
}

#[derive(Deserialize)]
pub struct UpdateBookDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub launched_at: Option<String>,
    pub cover_type: Option<String>,
    pub author: Option<String>,
    pub edition: Option<String>,
    pub language: Option<String>,
    pub genre: Option<String>,
    pub isbn_10_code: Option<i64>,
    pub isbn_13_code: Option<String>,
    pub publisher: Option<String>,
    pub pages: Option<i64>,
    pub dimentions: Option<String>,
}

impl From<&Row> for Book {
    fn from(row: &Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            launched_at: row.get("launched_at"),
            cover_type: row.get("cover_type"),
            author: row.get("author"),
            edition: row.get("edition"),
            language: row.get("language"),
            genre: row.get("genre"),
            isbn_10_code: row.get("isbn_10_code"),
            isbn_13_code: row.get("isbn_13_code"),
            publisher: row.get("publisher"),
            pages: row.get("pages"),
            dimentions: row.get("dimentions"),

            created_at: row.get::<&str, DateTime<Utc>>("created_at").to_string(),

            updated_at: row.get::<&str, DateTime<Utc>>("updated_at").to_string(),
        }
    }
}

// DB connection
pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;
