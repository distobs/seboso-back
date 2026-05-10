use axum::{Router, routing::get, Json};
use serde_json::{json, Value};
use crate::types::DbPool;
pub mod user_routes;
pub mod sebo_routes;
pub mod book_routes;

async fn index_route() -> Json<Value> {
        Json(json!({
                "/users": {
                        "GET /users?page=<>&per_page=<>": {
                                "Descrição": "Lista usuários, com paginação",
                                "Parâmetros (de URL)": {
                                        "page": "Página",
                                        "per_page": "Número de usuários por página",
                                }
                        }, 
                        "GET /users/{user_id}": {
                                "Descrição": "Retorna informações de usuário com base no ID",
                        },
                        "POST /users/": {
                                "Descrição": "Cria usuário",
                                "Parâmetros (JSON)": [
                                        "nome",
                                        "email",
                                        "login",
                                        "password",
                                        "cell_number",
                                        "is_activated",
                                ]
                        },
                        "POST /users/login": {
                                "Descrição": "Faz login e retorna token JWT",
                                "Parâmetros (JSON)": [
                                        "login",
                                        "password",
                                ],
                        },
                        "PUT /users/{user_id}": {
                                "Descrição": "Atualiza um usuário, necessita de token do dono da conta",
                                "Parâmetros (JSON)": [
                                        "name",
                                        "email",
                                        "login",
                                        "password",
                                        "cell_number",
                                        "is_activated",
                                ]
                        },
                        "DELETE /users/{user_id}": {
                                "Descrição": "Exclui usuário, necessita de token do dono da conta",
                        }
                },
                "/stores": {
                        "GET /stores?page=<>&per_page=<>": {
                                "Descrição": "Lista sebos, com paginação",
                                "Parâmetros (de URL)": {
                                        "page": "Página",
                                        "per_page": "Número de sebos por página",
                                }
                        }, 
                        "GET /sebos/{sebos_id}": {
                                "Descrição": "Retorna informações de um sebo com base no ID",
                        },
                        "POST /sebos/": {
                                "Descrição": "Cria sebo",
                                "Parâmetros (JSON)": [
                                        "name",
                                        "cnpj",
                                        "street",
                                        "number",
                                        "city",
                                        "state",
                                        "city_block",
                                        "cep",
                                        "workers",
                                ],
                                "Adicional: parâmetro workers": {
                                        "Descrição": "O parâmetro 'workers' deve ser uma lista de objetos com o formato descrito abaixo",
                                        "Formato": {
                                                "user_id": "ID do usuário a ser adicionado como funcionário do sebo",
                                                "role": "Cargo do funcionário dentro do sebo." 
                                        },
                                }
                        },
                        "PUT /sebos/{sebo_id}": {
                                "Função": "Atualiza um sebo, necessita de token de um funcionário com role 'worker' ou 'owner'",
                                "Parâmetros (JSON)": [
                                        "name",
                                        "cnpj",
                                        "street",
                                        "number",
                                        "city",
                                        "state",
                                        "city_block",
                                        "cep",
                                ]
                        },
                        "DELETE /sebos/{sebo_id}": "Exclui sebo, necessita de token de um funcionário com role 'worker' ou 'owner'",
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
