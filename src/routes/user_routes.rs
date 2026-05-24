use crate::{
    auth::{
        jwt_auth::{Claims, generate_jwt, jwt_middleware}, user_auth::user_auth
    }, models::{
        user_model::{CreateUser, LoginUser, UpdateUser, User},
    }, types::{
        db_types::DbPool, response_types::ApiResponse
    }, utils::pagination_utils::Pagination
};
use axum::{
    Json, Router,
    extract::{self, Extension, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
};
use bcrypt::{DEFAULT_COST, hash, verify};

/*
    GET /users?page=1&per_page=10 - Lista usuários, com paginação
*/
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

/*
    GET /users/{user_id} - Retorna informações de usuário com base no ID
*/
async fn get_user_id(Path(user_id): Path<i64>, State(pool): State<DbPool>) -> Json<User> {
    let conn = pool.get().await.unwrap();
    let row = conn
        .query_one("SELECT * FROM users WHERE id = $1", &[&(user_id as i64)])
        .await
        .unwrap();

    let user = User::from(&row);

    Json(user)
}

/*
    POST /users - Cria usuário
*/
async fn create_user(
    State(pool): State<DbPool>,
    extract::Json(payload): extract::Json<CreateUser>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    let password = payload.password;

    let hashed_password = hash(password, DEFAULT_COST).unwrap();

    conn.execute(
        "INSERT INTO users (name, email, login, password, cell_number)
         VALUES ($1, $2, $3, $4, $5)",
        &[
            &payload.name,
            &payload.email,
            &payload.login,
            &hashed_password,
            &payload.cell_number,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok()
}

/*
    PUT /users/{user_id} - Atualiza um usuário, necessita de token do
    dono da conta ou de um admin.
*/
async fn update_user(
    Path(user_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateUser>,
) -> impl IntoResponse {
    if !user_auth(claims, user_id) {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute(
        "UPDATE users
         SET name = COALESCE($1, name),
             email = COALESCE($2, email),
             login = COALESCE($3, login),
             password = COALESCE($4, password),
             cell_number = COALESCE($5, cell_number),
             is_activated = COALESCE($6, is_activated)
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

    ApiResponse::ok_msg(format!("Usuário {} modificado.", user_id))
}

/*
    DELETE /users/{user_id} - Exclui usuário, necessita de token do dono da
    conta ou de um admin.
*/
async fn delete_user(
    Path(user_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    if !user_auth(claims, user_id) {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await
        .unwrap();

    ApiResponse::ok_msg(format!("Usuário {} deletado.", &user_id))
}

/*
    POST /users/login - Faz login e retorna token JWT
*/
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
        return ApiResponse::err_msg("Usuário não encontrado.", StatusCode::NOT_FOUND);
    };

    let user = User::from(&row);

    let is_valid = verify(&payload.password, &user.pw_hash).unwrap_or(false);

    if is_valid {
        let token = generate_jwt(user.id, user.is_admin);

        ApiResponse::ok_msg(token)
    } else {
        ApiResponse::err_msg("Login ou senha incorretos", StatusCode::FORBIDDEN)
    }
}

/*============================================================================*/

pub fn make_user_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/login", post(login_user))
        .route("/users/{user_id}", get(get_user_id));

    let protected_routes = Router::new()
        .route("/users/{user_id}", put(update_user).delete(delete_user))
        .layer(middleware::from_fn(jwt_middleware));

    public_routes.merge(protected_routes)
}
