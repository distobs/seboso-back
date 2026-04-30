use axum::{
    Router,
    extract::{Query, State, Json, Path},
    routing::{get},
};
use crate::types::{ApiError, ApiResponse, CreateStore, DbPool, Pagination, Store};

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
) -> Result<Json<ApiResponse>, ApiError> {
    let mut conn = pool.get().await.map_err(|_| ApiError::BadReq("DB pool error".into()))?;
    let tran = conn.transaction().await?;

    let row = tran.query_one(
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
    .await?;
    
    let store_id: i64 = row.get("id");
    
    for owner_id in &payload.owners {
        tran.execute("INSERT INTO user_store (id_user, id_store, role) VALUES ($1, $2, 'owner')", &[&owner_id, &store_id]).await?;
    }
    
    for worker_id in &payload.workers {
        tran.execute("INSERT INTO user_store (id_user, id_store, role) VALUES ($1, $2, 'worker')", &[&worker_id, &store_id]).await?;
    }

    tran.commit().await?;

    Ok(Json(ApiResponse {
        success: true,
        message: "Sebo criado.".to_string(),
    }))
}

pub fn make_sebo_routes() -> Router<DbPool> {
    Router::new()
        .route("/stores", get(list_stores).post(create_store))
        .route("/stores/{store_id}", get(get_store_id))
}
