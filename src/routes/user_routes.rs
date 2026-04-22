use crate::types::{ApiResponse, CreateUser, DbPool, LoginUser, Pagination, User};
use axum::{
    Json, Router,
    extract::{self, Path, Query, State},
    response::IntoResponse,
    routing::{get, post},
};
use bcrypt::{DEFAULT_COST, hash, verify};

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

    let password = payload.password;

    // Gera o hash da senha
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    // DEFAULT_COST geralmente = 12 (bom equilíbrio segurança/performance)

    conn.execute(
        "INSERT INTO users (name, email, login, password, cell_number, role, is_activated)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &hashed_password,
            &payload.cell_number,
            &payload.role,
            &payload.is_activated,
        ],
    )
    .await
    .unwrap();

    Json(ApiResponse {
        success: true,
        message: "Usuário criado.".to_string(),
    })
}

async fn login_user(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginUser>,
) -> Json<ApiResponse> {
    let conn = pool.get().await.unwrap();

    let row = conn
        .query_one("SELECT * FROM users WHERE login = $1", &[&payload.login])
        .await
        .unwrap();

    let user = User::from(&row);

    let is_valid = verify(&payload.password, &user.pw_hash).unwrap();

    if is_valid {
        return Json(ApiResponse {
            success: true,
            message: "Usuário autenticado.".to_string(),
        });
    } else {
        return Json(ApiResponse {
            success: false,
            message: "Login ou Senha incorretos.".to_string(),
        });
    }
}

async fn update_user(
    Path(user_id): Path<i32>,
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "UPDATE users
         SET name = $1,
             email = $2,
             login = $3,
             password = $4,
             cell_number = $5,
             role = $6,
             is_activated = $7
         WHERE id = $8",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &payload.password,
            &payload.cell_number,
            &payload.role,
            &payload.is_activated,
            &user_id,
        ],
    )
    .await
    .unwrap();

    Json(ApiResponse {
        success: true,
        message: format!("Usuário {} modificado.", &payload.name).to_string(),
    })
}

async fn delete_user(Path(user_id): Path<i32>, State(pool): State<DbPool>) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await
        .unwrap();

    Json(ApiResponse {
        success: true,
        message: format!("Usuário {} deletado.", &user_id).to_string(),
    })
}

pub fn make_user_routes() -> Router<DbPool> {
    Router::new()
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/{user_id}",
            get(get_user_id).put(update_user).delete(delete_user),
        )
        .route("/users/login", post(login_user))
}
