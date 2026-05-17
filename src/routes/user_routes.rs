use crate::{
    types::{ApiResponse, CreateUser, DbPool, LoginUser, Pagination, User},
    utils::{Claims, jwt_middleware, load_env_vars},
};
use axum::{
    Json, Router,
    extract::{self, Extension, Path, Query, State},
    http::{StatusCode},
    middleware,
    response::{IntoResponse},
    routing::{get, post, put},
};
use bcrypt::{DEFAULT_COST, hash, verify};

use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};

fn generate_jwt(user_id: i64) -> String {
    let config = load_env_vars().unwrap();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
    };

    let secret = config.secret_key; // coloque em env depois!

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

async fn get_user_id(Path(user_id): Path<usize>, State(pool): State<DbPool>) -> Json<User> {
    let conn = pool.get().await.unwrap();
    let row = conn
        .query_one("SELECT * FROM users WHERE id = $1", &[&(user_id as i64)])
        .await
        .unwrap();

    let user = User::from(&row);

    Json(user)
}

// users?page=1&per_page=10
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

    let hashed_password = hash(password, DEFAULT_COST).unwrap();

    conn.execute(
        "INSERT INTO users (name, email, login, password, cell_number,is_activated)
         VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &hashed_password,
            &payload.cell_number,
            &payload.is_activated,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok()
}

async fn login_user(
    State(pool): State<DbPool>,
    Json(payload): Json<LoginUser>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    let row = conn
        .query_opt("SELECT * FROM users WHERE login = $1", &[&payload.login])
        .await
        .unwrap();

    let Some(row) = row else {
        return ApiResponse::err_msg(
            "Usuário não encontrado.",
            StatusCode::NOT_FOUND
        );
    };

    let user = User::from(&row);

    let is_valid = verify(&payload.password, &user.pw_hash).unwrap_or(false);

    if is_valid {
        let token = generate_jwt(user.id.try_into().unwrap());

        return ApiResponse::ok_msg(token);
    } else {
        return ApiResponse::err_msg(
            "Login ou senha incorretos",
            StatusCode::FORBIDDEN
        );
    }
}

async fn update_user(
    Path(user_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    if claims.sub != user_id {
        return ApiResponse::err(StatusCode::FORBIDDEN)
    }

    let conn = pool.get().await.unwrap();

    conn.execute(
        "UPDATE users
         SET name = $1,
             email = $2,
             login = $3,
             password = $4,
             cell_number = $5,
             is_activated = $6
         WHERE id = $7",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &payload.password,
            &payload.cell_number,
            &payload.is_activated,
            &user_id,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok_msg(format!("Usuário {} modificado.", &payload.name))
}

async fn delete_user(
    Path(user_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    if claims.sub != user_id {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await
        .unwrap();

    ApiResponse::ok_msg(format!("Usuário {} deletado.", &user_id))
}

pub fn make_user_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/login", post(login_user))
        .route("/users/{user_id}", get(get_user_id));

    let protected_routes = Router::new()
        .route(
            "/users/{user_id}",
                put(update_user)
                .delete(delete_user),
        )
        .layer(middleware::from_fn(jwt_middleware));

    public_routes.merge(protected_routes)
}
