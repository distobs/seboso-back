use axum::{
    Extension, Json, Router,
    extract::{self, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post, put},
};

use crate::{
    types::{ApiResponse, Catalog, CatalogQuery, DbPool, Pagination, UpdateCatalog},
    utils::{Claims, jwt_middleware},
};

// /catalog?page=1&per_page=10
async fn list_catalog(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Json<Vec<Catalog>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.unwrap();

    let rows = conn
        .query(
            "SELECT * FROM catalog ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .unwrap();

    let catalogs: Vec<Catalog> = rows.iter().map(Catalog::from).collect();

    Json(catalogs)
}

async fn list_catalog_by_store(
    Path(id_store): Path<usize>,
    State(pool): State<DbPool>,
) -> Json<Vec<Catalog>> {
    let conn = pool.get().await.unwrap();

    let rows = conn
        .query(
            "SELECT * FROM catalog WHERE id_store = $1",
            &[&(id_store as i64)],
        )
        .await
        .unwrap();

    let catalogs: Vec<Catalog> = rows.iter().map(Catalog::from).collect();

    Json(catalogs)
}

async fn create_book_in_catalog(
    State(pool): State<DbPool>,
    extract::Json(payload): extract::Json<Catalog>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "INSERT INTO catalog (id_store, isbn_10_code_book, price, quantity, description)
         VALUES ($1, $2, $3, $4, $5)",
        &[
            &payload.id_store,
            &payload.isbn_10_code_book,
            &payload.price,
            &payload.quantity,
            &payload.description,
        ],
    )
    .await
    .unwrap();

    Json(ApiResponse {
        success: true,
        message: "Produto criado.".to_string(),
    })
}

async fn update_book_in_catalog(
    Query(params): Query<CatalogQuery>,
    State(pool): State<DbPool>,
    Json(payload): Json<UpdateCatalog>,
) -> Result<impl IntoResponse, StatusCode> {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "
        UPDATE catalog
        SET isbn_10_code_book = $1,
            price = $2,
            quantity = $3,
            description = $4
        WHERE id_store = $5 AND isbn_10_code_book = $6
        ",
        &[
            &payload.isbn_10_code_book,
            &payload.price,
            &payload.quantity,
            &payload.description,
            &params.id_store,
            &params.isbn_10_code_book,
        ],
    )
    .await
    .unwrap();

    Ok(Json(ApiResponse {
        success: true,
        message: format!("Produto {} modificado.", &payload.isbn_10_code_book),
    }))
}

async fn delete_book_in_catalog(
    Query(params): Query<CatalogQuery>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    if claims.sub != params.id_store {
        return Err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute(
        "DELETE FROM catalog WHERE id_store = $1 AND isbn_10_code_book = $2",
        &[&params.id_store, &params.isbn_10_code_book],
    )
    .await
    .unwrap();

    Ok(Json(ApiResponse {
        success: true,
        message: format!(
            "Produto {} deletado do catalogo {}.",
            &params.isbn_10_code_book, &params.id_store
        )
        .to_string(),
    }))
}

pub fn make_catalog_routes() -> Router<DbPool> {
    Router::new()
        .route("/catalog", get(list_catalog))
        .route("/catalog/{id_store}", get(list_catalog_by_store))
        .route("/catalog", post(create_book_in_catalog))
        .route(
            "/catalog",
            put(update_book_in_catalog).delete(delete_book_in_catalog),
        )
        .layer(middleware::from_fn(jwt_middleware))
}
