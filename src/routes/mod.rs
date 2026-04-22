use axum::{Router, routing::get};
use crate::types::DbPool;
pub mod user_routes;
pub mod sebo_routes;
pub mod book_routes;

pub fn make_routes() -> Router<DbPool> {
        Router::new()
        .route("/", get(|| async { "/user" }))
        .merge(user_routes::make_user_routes())
        .merge(sebo_routes::make_sebo_routes())
        .merge(book_routes::make_book_routes())
}