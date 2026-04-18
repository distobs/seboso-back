use axum::{Router,
        routing::{get},
        extract::{State, Path},
        Json
};
use crate::types::{DbPool, User};

async fn get_user_id(
        Path(user_id): Path<usize>,
        State(pool): State<DbPool>
) -> Json<User> {
        let conn = pool.get().await.unwrap();
        let row = &conn
                .query_one(&format!(
                        "SELECT * FROM users WHERE id={}",
                        user_id
                ), &[]).await.unwrap();

        let user = User::from(row);

        Json(user)
}

pub fn make_user_routes() -> Router<DbPool> {
    Router::new().
        route("/user/{user_id}",
            get(get_user_id))
}