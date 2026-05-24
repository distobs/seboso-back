use axum::{
    Extension, Json, Router,
    extract::{self, Path, Query, State},
    http::StatusCode,
    middleware,
    routing::{get, post},
};

use crate::{
    models::{
        catalog_model::{Catalog, CatalogQuery, UpdateCatalog},
    },
    types::{
        response_types::ApiResponse,
        db_types:: DbPool
    },
    utils::pagination_utils::Pagination,
    auth::{
        jwt_auth::{Claims, jwt_middleware},
        catalog_auth::catalog_auth
    }
};

// /catalog?page=1&per_page=10
async fn list_catalog(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Catalog>>, ApiResponse> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let rows = conn
        .query(
            "SELECT * FROM catalog ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let catalogs: Vec<Catalog> = rows.iter().map(Catalog::from).collect();

    Ok(Json(catalogs))
}

async fn list_catalog_by_store(
    Path(store_id): Path<usize>,
    State(pool): State<DbPool>,
) -> Result<Json<Vec<Catalog>>, ApiResponse> {
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let rows = conn
        .query(
            "SELECT * FROM catalog WHERE store_id = $1",
            &[&(store_id as i64)],
        )
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    let catalogs: Vec<Catalog> = rows.iter().map(Catalog::from).collect();

    Ok(Json(catalogs))
}

async fn create_book_in_catalog(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    extract::Json(payload): extract::Json<Catalog>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = catalog_auth(&claims, payload.store_id, &pool)
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }
    
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute(
        "INSERT INTO catalog (store_id, book_id, price, quantity, description)
         VALUES ($1, $2, $3, $4, $5)",
        &[
            &payload.store_id,
            &payload.book_id,
            &payload.price,
            &payload.quantity,
            &payload.description,
        ],
    )
    .await
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg("Produto criado."))
}

async fn update_book_in_catalog(
    Query(params): Query<CatalogQuery>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateCatalog>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = catalog_auth(&claims, params.store_id, &pool).await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;
    
    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }
    
    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute(
        "
        UPDATE catalog
        SET price = COALESCE($1, price),
            quantity = COALESCE($2, quantity),
            description = COALESCE($3, description)
        WHERE store_id = $4 AND book_id = $5
        ",
        &[
            &payload.price,
            &payload.quantity,
            &payload.description,
            &params.store_id,
            &params.book_id,
        ],
    )
    .await
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!(
        "Produto {} modificado.",
        &payload.isbn_10_code_book
    )))
}

async fn delete_book_in_catalog(
    Query(params): Query<CatalogQuery>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = catalog_auth(&claims, params.store_id, &pool)
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute(
        "DELETE FROM catalog WHERE store_id = $1 AND book_id = $2",
        &[&params.store_id, &params.book_id],
    )
    .await
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!(
        "Produto {} deletado do catalogo {}.",
        &params.book_id, &params.store_id
    )))
}

pub fn make_catalog_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/catalog", get(list_catalog))
        .route("/catalog/{store_id}", get(list_catalog_by_store));

    let protected_routes = Router::new().route(
        "/catalog/",
        post(create_book_in_catalog)
            .put(update_book_in_catalog)
            .delete(delete_book_in_catalog)
            .layer(middleware::from_fn(jwt_middleware)),
    );

    public_routes.merge(protected_routes)
}
