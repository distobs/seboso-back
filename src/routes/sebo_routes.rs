use axum::Router;

use crate::types::DbPool;

pub fn make_sebo_routes() -> Router<DbPool> {
    Router::new()
}
