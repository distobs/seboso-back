use axum::{Router, routing::{get}};

async fn hello() -> &'static str {
        "hello user routes"
}

pub fn make_user_routes() -> Router {
    Router::new().
        route("/user",
            get(hello)
            .post(hello))
}