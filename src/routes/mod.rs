use axum::{Router, routing::get, Json};
use serde_json::{json, Value};
use crate::types::DbPool;
pub mod user_routes;
pub mod sebo_routes;
pub mod book_routes;

async fn index_route() -> Json<Value> {
        Json(json!({
                "/users": {
                        "GET /users?page=<>&per_page=<>": "Lista usuários", 
                        "GET /users/{user_id}": "Usuário por ID",
                        "POST /users/": {
                                "Função": "Cria usuário",
                                "Parâmetros": [
                                        "nome",
                                        "email",
                                        "login",
                                        "password",
                                        "cell_number",
                                        "is_activated",
                                ]
                        },
                        "POST /users/login": {
                                "Função": "Faz login (retorna token JWT)",
                                "Parâmetros": [
                                        "login",
                                        "password",
                                ],
                        },
                        "PUT /users/{user_id}": {
                                "Função": "Atualiza usuário",
                                "Parâmetros": [
                                        "nome",
                                        "email",
                                        "login",
                                        "password",
                                        "cell_number",
                                        "is_activated",
                                ]
                        },
                        "DELETE /users/{user_id}": "Exclui usuário",
                }
        })) 
}

pub fn make_routes() -> Router<DbPool> {
        Router::new()
        .route("/", get(index_route))
        .merge(user_routes::make_user_routes())
        .merge(sebo_routes::make_sebo_routes())
        .merge(book_routes::make_book_routes())
}