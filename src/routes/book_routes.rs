use axum::{
    Extension, Json, Router,
    extract::{self, Path, Query, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, put, post},
};

use crate::{
    types::{ApiResponse, Book, BookQuery, CreateBookDto, DbPool, Pagination, UpdateBookDto},
    utils::{Claims, jwt_middleware},
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

async fn get_book_isbn10(
    State(pool): State<DbPool>,
    Query(params): Query<BookQuery>,
) -> Json<Book> {
    let conn = pool.get().await.unwrap();

    let row = conn
        .query_one(
            "SELECT * FROM books WHERE isbn_10_code = $1",
            &[&params.isbn_10],
        )
        .await
        .unwrap();

    let book = Book::from(&row);

    Json(book)
}

async fn create_book(
    State(pool): State<DbPool>,
    extract::Json(payload): extract::Json<CreateBookDto>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "
        INSERT INTO books (
            title,
            description,
            launched_at,
            cover_type,
            author,
            edition,
            language,
            genre,
            isbn_10_code,
            isbn_13_code,
            publisher,
            pages,
            dimentions
        )
        VALUES (
            $1, $2, $3, $4, $5, $6,
            $7, $8, $9, $10, $11, $12, $13
        )
        ",
        &[
            &payload.title,
            &payload.description,
            &payload.launched_at,
            &payload.cover_type,
            &payload.author,
            &payload.edition,
            &payload.language,
            &payload.genre,
            &payload.isbn_10_code,
            &payload.isbn_13_code,
            &payload.publisher,
            &payload.pages,
            &payload.dimentions,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok_msg("Livro criado.")
}

async fn update_book(
    Path(book_id): Path<i64>,
    State(pool): State<DbPool>,
    Json(payload): Json<UpdateBookDto>,
) -> impl IntoResponse {
    let conn = pool.get().await.unwrap();

    conn.execute(
        "
        UPDATE books
        SET title = $1,
            description = $2,
            launched_at = $3,
            cover_type = $4,
            author = $5,
            edition = $6,
            language = $7,
            genre = $8,
            isbn_10_code = $9,
            isbn_13_code = $10,
            publisher = $11,
            pages = $12,
            dimentions = $13
        WHERE id = $14
        ",
        &[
            &payload.title,
            &payload.description,
            &payload.launched_at,
            &payload.cover_type,
            &payload.author,
            &payload.edition,
            &payload.language,
            &payload.genre,
            &payload.isbn_10_code,
            &payload.isbn_13_code,
            &payload.publisher,
            &payload.pages,
            &payload.dimentions,
            &book_id,
        ],
    )
    .await
    .unwrap();

    ApiResponse::ok_msg(format!("Livro {:?} modificado.", &payload.title))
}

async fn delete_book(
    Path(book_id): Path<i64>,
    State(pool): State<DbPool>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    if claims.sub != book_id {
        return ApiResponse::err(StatusCode::FORBIDDEN);
    }

    let conn = pool.get().await.unwrap();

    conn.execute("DELETE FROM books WHERE id = $1", &[&book_id])
        .await
        .unwrap();

    ApiResponse::ok_msg(format!("Livro {} deletado.", &book_id))
}

pub fn make_book_routes() -> Router<DbPool> {
    let public_routes = Router::new()
        .route("/books", get(list_books))
        .route("/books/isbn", get(get_book_isbn10));
    
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
