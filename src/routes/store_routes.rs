use crate::{
    auth::{
        jwt_auth::{Claims, jwt_middleware}, store_auth::store_auth
    }, models::store_model::{
        CreateStore,
        Store,
        UpdateStore
    }, types::{
        db_types::DbPool, response_types::ApiResponse
    }, utils::{pagination_utils::Pagination}
};

use axum::{
    Extension, Router,
    extract::{Json, Path, Query, State},
    http::StatusCode,
    middleware,
    routing::{delete, get, post},
};


/*
    GET /stores?page=1&per_page=10 - Lista usuários, com paginação
*/
async fn list_stores(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Store>>, ApiResponse> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let rows = conn
        .query(
            "SELECT * FROM stores ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let stores: Vec<Store> = rows.iter().map(Store::from).collect();

    Ok(Json(stores))
}

/*
    GET /stores/{id} - Obtém um sebo pelo ID
*/
async fn get_store_id(Path(store_id): Path<i64>, State(pool): State<DbPool>) -> Result<Json<Store>, ApiResponse> {
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let row = conn
        .query_one("SELECT * FROM stores WHERE id = $1", &[&(store_id as i64)])
        .await
        .map_err(|_| ApiResponse::err(StatusCode::NOT_FOUND))?;

    let store = Store::from(&row);

    Ok(Json(store))
}

/*
    POST /stores - Cria um novo sebo, necessita de token de usuário autenticado.
*/
async fn create_store(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateStore>,
) -> Result<ApiResponse, ApiResponse> {
    let mut conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let tran = conn.transaction().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

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
        .await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let store_id: i64 = row.get("id");

    let owner_id = claims.sub;

    tran.execute(
        "INSERT INTO user_store (user_id, store_id, role) VALUES ($1, $2, $3)",
        &[&owner_id, &store_id, &"owner"],
    )
    .await
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    tran.commit().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg("Sebo criado."))
}

/*
    PUT /stores/{id} - Atualiza um sebo, necessita de token de um funcionário com role 'worker' ou 'owner'.
*/
async fn update_store(
    Path(store_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateStore>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = store_auth(&claims, store_id, &pool)
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute(
        "UPDATE stores
         SET name = COALESCE($1, name),
             cnpj = COALESCE($2, cnpj),
             street = COALESCE($3, street),
             number = COALESCE($4, number),
             city = COALESCE($5, city),
             state = COALESCE($6, state),
             city_block = COALESCE($7, city_block),
             cep = COALESCE($8, cep)
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
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!("Sebo {} modificado.", &store_id)))
}

/*
    DELETE /stores/{id} - Exclui um sebo, necessita de token de um funcionário
    com role 'worker' ou 'owner'.
*/
async fn delete_store(
    Path(store_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = store_auth(&claims, store_id, &pool)
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get()
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute("DELETE FROM stores WHERE id = $1", &[&store_id])
        .await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!("Sebo {} deletado.", &store_id)))
}

/*============================================================================*/

pub fn make_store_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/stores", get(list_stores))
        .route("/stores/{store_id}", get(get_store_id));

    let protected_routes = Router::new().route(
        "/stores",
        post(create_store)
    )
    .route(
        "/stores/{store_id}",
            delete(delete_store)
            .put(update_store)
    ).layer(middleware::from_fn(jwt_middleware));

    public_routes.merge(protected_routes)
}
