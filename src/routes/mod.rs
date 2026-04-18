use axum::{Router, routing::get};
pub mod user_routes;

pub fn make_routes() -> Router {
        Router::new()
        .route("/", get(|| async { "/user" }))
        .merge(user_routes::make_user_routes())
}