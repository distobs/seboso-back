use axum::{
    Extension, Json, Router,
    extract::{self, Path, Query, State},
    http::StatusCode,
    middleware,
    routing::{get, put, post},
};

use crate::{
    auth::{
        book_auth::book_auth, jwt_auth::{Claims, jwt_middleware}, 
    }, 
    utils::pagination_utils::Pagination,
    models:: book_model::{
        Book, CreateBook, UpdateBook
    },
    types::{
        db_types::DbPool, response_types::ApiResponse
    }
};

// books?page=1&per_page=10
async fn list_books(
    Query(pagination): Query<Pagination>,
    State(pool): State<DbPool>,
) -> Json<Vec<Book>> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);

    let offset = (page - 1) * per_page;

    let conn = pool.get().await.unwrap();

    let rows = conn
        .query(
            "SELECT * FROM books ORDER BY id LIMIT $1 OFFSET $2",
            &[&(per_page as i64), &(offset as i64)],
        )
        .await
        .unwrap();

    let books: Vec<Book> = rows.iter().map(Book::from).collect();

    Json(books)
}

async fn get_book_id(
    Path(book_id): Path<i64>,
    State(pool): State<DbPool>,
) -> Json<Book> {
    let conn = pool.get().await.unwrap();

    let row = conn
        .query_one(
            "SELECT * FROM books WHERE id = $1",
            &[&book_id],
        )
        .await
        .unwrap();

    let book = Book::from(&row);

    Json(book)
}

async fn create_book(
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    extract::Json(payload): extract::Json<CreateBook>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = book_auth(&claims).await;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get().await.unwrap();

    conn.execute(
        "
        INSERT INTO books (
            title,
            author,
            description,
            published_at,
            cover_type,
            edition,
            language,
            genre,
            isbn_10_code,
            isbn_13_code,
            publisher,
            pages,
            dimensions
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13
        )
        ",
        &[
            &payload.title,
            &payload.author,
            &payload.description,
            &payload.published_at,
            &payload.cover_type,
            &payload.edition,
            &payload.language,
            &payload.genre,
            &payload.isbn_10_code,
            &payload.isbn_13_code,
            &payload.publisher,
            &payload.pages,
            &payload.dimensions,
        ],
    )
    .await
    .unwrap();

    Ok(ApiResponse::ok_msg("Livro criado."))
}

async fn update_book(
    Path(book_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateBook>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = book_auth(&claims).await;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute(
        "
        UPDATE books
        SET title = COALESCE($1, title),
            author = COALESCE($2, author),
            description = COALESCE($3, description),
            published_at = COALESCE($4, published_at),
            cover_type = COALESCE($5, cover_type),
            edition = COALESCE($6, edition),
            language = COALESCE($7, language),
            genre = COALESCE($8, genre),
            isbn_10_code = COALESCE($9, isbn_10_code),
            isbn_13_code = COALESCE($10, isbn_13_code),
            publisher = COALESCE($11, publisher),
            pages = COALESCE($12, pages),
            dimensions = COALESCE($13, dimensions)
        WHERE id = $14
        ",
        &[
            &payload.title,
            &payload.author,
            &payload.description,
            &payload.published_at,
            &payload.cover_type,
            &payload.edition,
            &payload.language,
            &payload.genre,
            &payload.isbn_10_code,
            &payload.isbn_13_code,
            &payload.publisher,
            &payload.pages,
            &payload.dimensions,
            &book_id,
        ],
    )
    .await
    .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!("Livro {:?} modificado.", &payload.title)))
}

async fn delete_book(
    Path(book_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> Result<ApiResponse, ApiResponse> {
    let authorized = book_auth(&claims).await;

    if !authorized {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    if claims.sub != book_id {
        return Err(ApiResponse::err(StatusCode::FORBIDDEN));
    }

    let conn = pool.get().await.map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    conn.execute("DELETE FROM books WHERE id = $1", &[&book_id])
        .await
        .map_err(|_| ApiResponse::err(StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(ApiResponse::ok_msg(format!("Livro {} deletado.", &book_id)))
}

pub fn make_book_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/books", get(list_books))
        .route("/books/{book_id}", get(get_book_id));
    
    let protected_routes = Router::new()
        .route(
            "/books",
                post(create_book)
                .layer(middleware::from_fn(jwt_middleware))
        )
        .route(
            "/books/{book_id}",
                put(update_book)
                .delete(delete_book)
                .layer(middleware::from_fn(jwt_middleware))
        );

    public_routes.merge(protected_routes)
}
