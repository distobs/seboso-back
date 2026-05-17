use crate::{
    types::{ApiResponse, CreateStore, DbPool, Pagination, Store, UserStore},
    utils::{Claims, jwt_middleware},
};
use axum::{
    Extension, Router,
    extract::{Json, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
};

async fn get_store_id(Path(store_id): Path<usize>, State(pool): State<DbPool>) -> Json<Store> {
    let conn = pool.get().await.unwrap();

    let row = conn
        .query_one("SELECT * FROM stores WHERE id = $1", &[&(store_id as i64)])
        .await
        .unwrap();

    let store = Store::from(&row);

    Json(store)
}

/// stores?page=1&per_page=10
async fn list_stores(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Json<Vec<Store>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.unwrap();

    let rows = conn
        .query(
            "SELECT * FROM stores ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .unwrap();

    let stores: Vec<Store> = rows.iter().map(Store::from).collect();

    Json(stores)
}

async fn create_store(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateStore>,
) -> impl IntoResponse {
    let mut conn = pool.get().await.unwrap();

    let tran = conn.transaction().await.unwrap();

    let row = tran
        .query_one(
            "INSERT INTO stores (name, cnpj, street, number, city, state, city_block, cep)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id",
            &[
                &payload.name,
                &payload.cnpj,
                &payload.street,
                &payload.number,
                &payload.city,
                &payload.state,
                &payload.city_block,
                &payload.cep,
            ],
        )
        .await.unwrap();

    let store_id: i64 = row.get("id");

    for worker in &payload.workers {
        tran.execute(
            "INSERT INTO user_store (id_user, id_store, role) VALUES ($1, $2, $3)",
            &[&worker.user_id, &store_id, &worker.role],
        )
        .await.unwrap();
    }

    tran.commit().await.unwrap();

    ApiResponse::ok_msg("Sebo criado.")
}

async fn update_store(
    Path(store_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStore>,
) -> impl IntoResponse {
    if !UserStore::check_role_in_store(claims.sub, store_id, &["worker", "owner"], &pool)
        .await.unwrap()
    {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute(
        "UPDATE stores
         SET name = $1,
             cnpj = $2,
             street = $3,
             number = $4,
             city = $5,
             state = $6,
             city_block = $7,
             cep = $8
         WHERE id = $9",
        &[
            &payload.name,
            &payload.cnpj,
            &payload.street,
            &payload.number,
            &payload.city,
            &payload.state,
            &payload.city_block,
            &payload.cep,
            &store_id,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok_msg(format!("Sebo {} modificado.", &payload.name))
}

async fn delete_store(
    Path(store_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    if !UserStore::check_role_in_store(claims.sub, store_id, &["worker", "owner"], &pool)
        .await
        .unwrap()
    {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute("DELETE FROM stores WHERE id = $1", &[&store_id])
        .await
        .unwrap();

    ApiResponse::ok_msg(format!("Sebo {} deletado.", &store_id))
}

pub fn make_sebo_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/stores", get(list_stores))
        .route("/stores/{store_id}", get(get_store_id))
        .route("/stores", post(create_store));

    let protected_routes = Router::new()
        .route(
            "/stores/{store_id}",
                put(update_store)
                .delete(delete_store)
                .layer(middleware::from_fn(jwt_middleware))
        );

    public_routes.merge(protected_routes)
}
