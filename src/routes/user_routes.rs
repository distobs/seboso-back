use crate::types::{ApiResponse, CreateUser, DbPool, Pagination, User};
use axum::{
    Json, Router,
    extract::{self, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

// TODO: implement a route to get all users that works efficiently
//       (that is, a route that doesn't fill our memory with users)
async fn get_user_id(Path(user_id): Path<usize>, State(pool): State<DbPool>) -> Json<User> {
    let conn = pool.get().await.unwrap();
    let row = conn
        .query_one("SELECT * FROM users WHERE id = $1", &[&(user_id as i32)])
        .await
        .unwrap();

    let user = User::from(&row);

    Json(user)
}

/// users?page=1&per_page=10
async fn list_users(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Json<Vec<User>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.unwrap();

    let rows = conn
        .query(
            "SELECT * FROM users ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .unwrap();

    let users: Vec<User> = rows.iter().map(User::from).collect();

    Json(users)
}

async fn create_user(
    State(pool): State<DbPool>,
    extract::Json(payload): extract::Json<CreateUser>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "INSERT INTO users (name, email, login, password, cell_number, role)
         VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &payload.pw_hash,
            &payload.cell_number,
            &payload.role,
        ],
    )
    .await
    .unwrap();

    Json(ApiResponse {
        success: true,
        message: "Usuário criado.".to_string(),
    })
}

pub fn make_user_routes() -> Router<DbPool> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/{user_id}", get(get_user_id))
}
