use axum::{Router, routing::{get}};

/*
use chrono::{DateTime, Utc};
use tokio_postgres::{Row};

struct User {
    id: i32,
    name: String,
    email: String,
    login: String,
    pw_hash: String,
    cell_number: String,
    role: i64,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<Row> for User {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            login: row.get("login"),
            pw_hash: row.get("password"),
            cell_number: row.get("cell_number"),
            role: row.get("role"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
*/

async fn hello() -> &'static str {
        "hello user routes"
}

pub fn make_user_routes() -> Router {
    Router::new().
        route("/user",
            get(hello)
            .post(hello))
}