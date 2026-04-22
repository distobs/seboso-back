use axum::Router;

use crate::types::DbPool;

pub fn make_book_routes() -> Router<DbPool> {
    Router::new()
}
