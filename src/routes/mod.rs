use crate::{types::db_types::DbPool};
use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
pub mod book_routes;
pub mod catalog_routes;
pub mod store_routes;
pub mod user_routes;
pub mod userstore_routes;

async fn index_route() -> Json<Value> {
    Json(json!({
        "DICA": "parâmetros marcados com '?' são opcionais",
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
                    "cell_number?",
                ]
            },
            "POST /users/login": {
                "Descrição": "Faz login e retorna token JWT",
                "Retornos": {
                    "200": "Login bem-sucedido, retorna token JWT",
                    "403": "Credenciais inválidas",
                    "404": "Usuário não encontrado",
                },
                "Parâmetros (JSON)": [
                    "login",
                    "password",
                ],
            },
            "PUT /users/{user_id}": {
                "Descrição": "Atualiza um usuário",
                "Permissões": "Usuário deve ser dono da conta ou admin.",
                "Retornos": {
                    "200": "Atualização bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "name?",
                    "email?",
                    "login?",
                    "password",
                    "cell_number?",
                    "is_activated?",
                ]
            },
            "DELETE /users/{user_id}": {
                "Descrição": "Exclui usuário.",
                "Permissões": "Usuário deve ser dono da conta ou admin.",
                "Retornos": {
                    "200": "Exclusão bem-sucedida",
                    "403": "Permissão negada",
                }
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
                "Permissões": "Necessita de token de usuário logado",
                "Retornos": {
                    "200": "Criação bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "name",
                    "cnpj",
                    "street",
                    "number",
                    "city",
                    "state",
                    "city_block",
                    "cep",
                ],
            },

            "PUT /sebos/{sebo_id}": {
                "Função": "Atualiza um sebo",
                "Permissões": "Funcionário com role 'worker' ou 'owner', ou admin",
                "Retornos": {
                    "200": "Atualização bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "name?",
                    "cnpj?",
                    "street?",
                    "number?",
                    "city?",
                    "state?",
                    "city_block?",
                    "cep?",
                ]
            },

            "DELETE /sebos/{sebo_id}": {
                "Descrição": "Exclui sebo",
                "Permissões": "Necessita de token de um funcionário com role 'worker' ou 'owner', ou admin",
                "Retornos": {
                    "200": "Exclusão bem-sucedida",
                    "403": "Permissão negada",
                }
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

            "GET /books/{book_id}": {
                "Descrição": "Retorna informações de um livro específico.",
            },

            "POST /books/": {
                "Descrição": "Cria livro",
                "Permissões": "Usuário deve ser admin.",
                "Retornos": {
                    "200": "Criação bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "title",
                    "author",
                    "description?",
                    "published_at?",
                    "cover_url?",
                    "cover_type?",
                    "edition?",
                    "language?",
                    "genre?",
                    "isbn_10_code?",
                    "isbn_13_code?",
                    "publisher?",
                    "pages?",
                    "dimensions?",
                ]
            },

            "PUT /books/{book_id}": {
                "Descrição": "Atualiza um livro",
                "Permissões": "Usuário deve ser admin.",
                "Retornos": {
                    "200": "Atualização bem-sucedida",
                    "403": "Permissão negada",
                },
                
                "Parâmetros (JSON)": [
                    "title?",
                    "description?",
                    "published_at?",
                    "cover_url?",
                    "cover_type?",
                    "author?",
                    "edition?",
                    "language?",
                    "genre?",
                    "isbn_10_code?",
                    "isbn_13_code?",
                    "publisher?",
                    "pages?",
                    "dimensions?",
                ]
            },

            "DELETE /books/{book_id}": {
                "Descrição": "Exclui livro",
                "Permissões": "Usuário deve ser admin.",
                "Retornos": {
                    "200": "Exclusão bem-sucedida",
                    "403": "Permissão negada",
                },
            }
        },

        "/catalog": {
            "GET /catalog": {
                "Descrição": "Lista todos os livros presentes no catálogo",
            },

            "GET /catalog/{store_id}": {
                "Descrição": "Lista todos os livros do catálogo de um sebo específico",
                "Parâmetros (de URL)": {
                    "store_id": "ID do sebo",
                }
            },

            "POST /catalog": {
                "Descrição": "Adiciona um livro ao catálogo de um sebo",
                "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
                "Retornos": {
                    "200": "Criação bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "store_id",
                    "book_id",
                    "price",
                    "quantity",
                    "state",
                ]
            },

            "PUT /catalog": {
                "Descrição": "Atualiza informações de um livro no catálogo",
                "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
                "Retornos": {
                    "200": "Atualização bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "store_id?",
                    "book_id?",
                    "price?",
                    "quantity?",
                    "state?",
                ]
            },

            "DELETE /catalog": {
                "Descrição": "Remove um livro do catálogo",
                "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
                "Retornos": {
                    "200": "Remoção bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "store_id",
                    "book_id",
                ]
            }
        },

        "/userstore": {
            "GET /userstore": {
               "Descrição": "Lista relações sebo-usuário.",
               "Parâmetros (de URL)": {
                    "store_id?": "ID da loja",
                    "user_id?": "ID de usuário",
                    "role?": "Role procurado"
                },
                "Funcionamento": "Use teoria dos conjuntos."
            },
            "POST /userstore": {
                "Descrição": "Cria relação sebo-usuário, ou seja, atribui uma role a um usuário em um sebo.",
                "Permissões": "Necessita de token do dono da conta ou admin",
                "Retornos": {
                    "200": "Criação bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "user_id",
                    "store_id",
                    "role"
                ]
            },
            "PUT /userstore": {
                "Descrição": "Atualiza relação sebo-usuário, ou seja, a role de um usuário em um sebo.",
                "Permissões": "Necessita de token do dono da conta ou admin",
                "Retornos": {
                    "200": "Atualização bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "user_id",
                    "store_id",
                    "role"
                ]
            },
            "DELETE /userstore": {
                "Descrição": "Remove relação sebo-usuário.",
                "Permissões": "Necessita de token do dono da conta ou admin",
                "Retornos": {
                    "200": "Remoção bem-sucedida",
                    "403": "Permissão negada",
                },
                "Parâmetros (JSON)": [
                    "user_id",
                    "store_id",
                    "role"
                ]
            },
        },
    }))
}

pub fn make_routes() -> Router<DbPool> {
    Router::new()
        .route("/", get(index_route))
        .merge(user_routes::make_user_routes())
        .merge(store_routes::make_store_routes())
        .merge(book_routes::make_book_routes())
        .merge(catalog_routes::make_catalog_routes())
        .merge(userstore_routes::make_userstore_routes())
}
