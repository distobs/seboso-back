use crate::types::DbPool;
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
pub mod book_routes;
pub mod sebo_routes;
pub mod user_routes;
pub mod catalog_routes;

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

            "DELETE /sebos/{sebo_id}": {
                "Descrição": "Exclui sebo, necessita de token de um funcionário com role 'worker' ou 'owner'",
            },
        },

        "/books": {
            "GET /books?page=<>&per_page=<>": {
                "Descrição": "Lista livros, com paginação",
                "Parâmetros (de URL)": {
                    "page": "Página",
                    "per_page": "Número de livros por página",
                }
            },

            "GET /books/{isbn_10}": {
                "Descrição": "Retorna informações de um livro com base no isbn_10 code",
            },

            "POST /books/": {
                "Descrição": "Cria livro",
                "Parâmetros (JSON)": [
                    "title",
                    "description",
                    "launched_at",
                    "cover_type",
                    "author",
                    "edition",
                    "language",
                    "genre",
                    "isbn_10_code",
                    "isbn_13_code",
                    "publisher",
                    "pages",
                    "dimentions",
                ]
            },

            "PUT /books/{book_id}": {
                "Descrição": "Atualiza um livro",
                "Parâmetros (JSON)": [
                    "title",
                    "description",
                    "launched_at",
                    "cover_type",
                    "author",
                    "edition",
                    "language",
                    "genre",
                    "isbn_10_code",
                    "isbn_13_code",
                    "publisher",
                    "pages",
                    "dimentions",
                ]
            },

            "DELETE /books/{book_id}": {
                "Descrição": "Exclui livro",
            }
        },

        "/catalog": {
            "GET /catalog": {
                "Descrição": "Lista todos os livros presentes no catálogo",
            },

            "GET /catalog/{id_store}": {
                "Descrição": "Lista todos os livros do catálogo de um sebo específico",
                "Parâmetros (de URL)": {
                    "id_store": "ID do sebo",
                }
            },

            "POST /catalog": {
                "Descrição": "Adiciona um livro ao catálogo de um sebo, necessita autenticação JWT",
                "Parâmetros (JSON)": [
                    "store_id",
                    "book_id",
                    "price",
                    "quantity",
                    "state",
                ]
            },

            "PUT /catalog": {
                "Descrição": "Atualiza informações de um livro no catálogo, necessita autenticação JWT",
                "Parâmetros (JSON)": [
                    "store_id",
                    "book_id",
                    "price",
                    "quantity",
                    "state",
                ]
            },

            "DELETE /catalog": {
                "Descrição": "Remove um livro do catálogo, necessita autenticação JWT",
                "Parâmetros (JSON)": [
                    "store_id",
                    "book_id",
                ]
            }
        }
    }))
}

pub fn make_routes() -> Router<DbPool> {
        Router::new()
        .route("/", get(index_route))
        .merge(user_routes::make_user_routes())
        .merge(sebo_routes::make_sebo_routes())
        .merge(book_routes::make_book_routes())
        .merge(catalog_routes::make_catalog_routes())
}
