# Back-end do Seboso

## O que é?

O Seboso quer fornecer uma interface prática para a descoberta e o gerenciamento
de sebos, visando ampliar a cultura do sebo. Este é o back-end - o servidor - do
Seboso.

## Como rodar?

No Linux:

- Clone o repositório:

```bash
$ git clone https://github.com/distobs/storeso-back.git
```

- Crie um .env:

```bash
$ cp .env.example .env
$ # abra o .env com seu editor favorito e preencha com os valores que quiser
```

- Instale o Docker e verifique se o comando `docker compose version` funciona.

- Rode e tome café enquanto compila :D

```bash
$ docker compose up --build # Para compilar e iniciar. d (de detach) para deixar em segundo plano.
$ docker compose down -v # para parar o container
$ # às vezes é necessário rodar como root. Se der erro, tente isso.
$ # pode acontecer do volume bugar e permanecer dados passados, ocasionando em erro. Se isso acontecer, remova os volumes no docker antes de rodar.
```

No Windows: deve ser parecido, mas não tenho um Windows instalado pra testar.
 O documento está aberto para melhor detalhamento.

## Como testar

No projeto, existe um script escrito em Python que cria um usuário administrador e faz um workflow genérico de CRUD usando as rotas da API.

O script está disponível no diretório `/scripts`. Certifique-se de que o back-end está de pé e, no Linux:

```bash
$ cd /scripts
$ python -m venv .venv
$ pip install psycopg2 requests python-dotenv
$ python populate_and_test.py
```

No Windows: WSL, Cygwin e adjacentes devem resolver. Fazer os requests manualmente também.

## Documentação das rotas criadas até o momento em que o JSON foi atualizado.

- Nota para o front-end: é bem provável que vocês acabem descobrindo alguns
erros com nosso back-end.

```json
{
  "/books": {
    "DELETE /books/{book_id}": {
      "Descrição": "Exclui livro",
      "Permissões": "Usuário deve ser admin.",
      "Retornos": {
        "200": "Exclusão bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "GET /books/{book_id}": {
      "Descrição": "Retorna informações de um livro específico."
    },
    "GET /books?page=<>&per_page=<>": {
      "Descrição": "Lista livros, com paginação",
      "Parâmetros (de URL)": {
        "page": "Página",
        "per_page": "Número de livros por página"
      }
    },
    "POST /books/": {
      "Descrição": "Cria livro",
      "Parâmetros (JSON)": [
        "title",
        "author",
        "description?",
        "published_at?",
        "cover_type?",
        "edition?",
        "language?",
        "genre?",
        "isbn_10_code?",
        "isbn_13_code?",
        "publisher?",
        "pages?",
        "dimensions?"
      ],
      "Permissões": "Usuário deve ser admin.",
      "Retornos": {
        "200": "Criação bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "PUT /books/{book_id}": {
      "Descrição": "Atualiza um livro",
      "Parâmetros (JSON)": [
        "title?",
        "description?",
        "published_at?",
        "cover_type?",
        "author?",
        "edition?",
        "language?",
        "genre?",
        "isbn_10_code?",
        "isbn_13_code?",
        "publisher?",
        "pages?",
        "dimensions?"
      ],
      "Permissões": "Usuário deve ser admin.",
      "Retornos": {
        "200": "Atualização bem-sucedida",
        "403": "Permissão negada"
      }
    }
  },
  "/catalog": {
    "DELETE /catalog": {
      "Descrição": "Remove um livro do catálogo",
      "Parâmetros (JSON)": [
        "store_id",
        "book_id"
      ],
      "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
      "Retornos": {
        "200": "Remoção bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "GET /catalog": {
      "Descrição": "Lista todos os livros presentes no catálogo"
    },
    "GET /catalog/{store_id}": {
      "Descrição": "Lista todos os livros do catálogo de um sebo específico",
      "Parâmetros (de URL)": {
        "store_id": "ID do sebo"
      }
    },
    "POST /catalog": {
      "Descrição": "Adiciona um livro ao catálogo de um sebo",
      "Parâmetros (JSON)": [
        "store_id",
        "book_id",
        "price",
        "quantity",
        "state"
      ],
      "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
      "Retornos": {
        "200": "Criação bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "PUT /catalog": {
      "Descrição": "Atualiza informações de um livro no catálogo",
      "Parâmetros (JSON)": [
        "store_id?",
        "book_id?",
        "price?",
        "quantity?",
        "state?"
      ],
      "Permissões": "Necessita de token de 'worker' ou 'owner' do sebo, ou admin",
      "Retornos": {
        "200": "Atualização bem-sucedida",
        "403": "Permissão negada"
      }
    }
  },
  "/stores": {
    "DELETE /stores/{sebo_id}": {
      "Descrição": "Exclui sebo",
      "Permissões": "Necessita de token de um funcionário com role 'worker' ou 'owner', ou admin",
      "Retornos": {
        "200": "Exclusão bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "GET /stores/{sebos_id}": {
      "Descrição": "Retorna informações de um sebo com base no ID"
    },
    "GET /stores?page=<>&per_page=<>": {
      "Descrição": "Lista sebos, com paginação",
      "Parâmetros (de URL)": {
        "page": "Página",
        "per_page": "Número de sebos por página"
      }
    },
    "POST /stores/": {
      "Descrição": "Cria sebo",
      "Parâmetros (JSON)": [
        "name",
        "cnpj",
        "street",
        "number",
        "city",
        "state",
        "city_block",
        "cep"
      ],
      "Permissões": "Necessita de token de usuário logado",
      "Retornos": {
        "200": "Criação bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "PUT /stores/{sebo_id}": {
      "Função": "Atualiza um sebo",
      "Parâmetros (JSON)": [
        "name?",
        "cnpj?",
        "street?",
        "number?",
        "city?",
        "state?",
        "city_block?",
        "cep?"
      ],
      "Permissões": "Funcionário com role 'worker' ou 'owner', ou admin",
      "Retornos": {
        "200": "Atualização bem-sucedida",
        "403": "Permissão negada"
      }
    }
  },
  "/users": {
    "DELETE /users/{user_id}": {
      "Descrição": "Exclui usuário.",
      "Permissões": "Usuário deve ser dono da conta ou admin.",
      "Retornos": {
        "200": "Exclusão bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "GET /users/{user_id}": {
      "Descrição": "Retorna informações de usuário com base no ID"
    },
    "GET /users?page=<>&per_page=<>": {
      "Descrição": "Lista usuários, com paginação",
      "Parâmetros (de URL)": {
        "page": "Página",
        "per_page": "Número de usuários por página"
      }
    },
    "POST /users/": {
      "Descrição": "Cria usuário",
      "Parâmetros (JSON)": [
        "nome",
        "email",
        "login",
        "password",
        "cell_number?"
      ]
    },
    "POST /users/login": {
      "Descrição": "Faz login e retorna token JWT",
      "Parâmetros (JSON)": [
        "login",
        "password"
      ],
      "Retornos": {
        "200": "Login bem-sucedido, retorna token JWT",
        "403": "Credenciais inválidas",
        "404": "Usuário não encontrado"
      }
    },
    "PUT /users/{user_id}": {
      "Descrição": "Atualiza um usuário",
      "Parâmetros (JSON)": [
        "name?",
        "email?",
        "login?",
        "password",
        "cell_number?",
        "is_activated?"
      ],
      "Permissões": "Usuário deve ser dono da conta ou admin.",
      "Retornos": {
        "200": "Atualização bem-sucedida",
        "403": "Permissão negada"
      }
    }
  },
  "/userstore": {
    "DELETE /userstore": {
      "Descrição": "Remove relação sebo-usuário.",
      "Parâmetros (JSON)": [
        "user_id",
        "store_id",
        "role"
      ],
      "Permissões": "Necessita de token do dono da conta ou admin",
      "Retornos": {
        "200": "Remoção bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "GET /userstore": {
      "Descrição": "Lista relações sebo-usuário.",
      "Funcionamento": "Use teoria dos conjuntos.",
      "Parâmetros (de URL)": {
        "role?": "Role procurado",
        "store_id?": "ID da loja",
        "user_id?": "ID de usuário"
      }
    },
    "POST /userstore": {
      "Descrição": "Cria relação sebo-usuário, ou seja, atribui uma role a um usuário em um sebo.",
      "Parâmetros (JSON)": [
        "user_id",
        "store_id",
        "role"
      ],
      "Permissões": "Necessita de token do dono da conta ou admin",
      "Retornos": {
        "200": "Criação bem-sucedida",
        "403": "Permissão negada"
      }
    },
    "PUT /userstore": {
      "Descrição": "Atualiza relação sebo-usuário, ou seja, a role de um usuário em um sebo.",
      "Parâmetros (JSON)": [
        "user_id",
        "store_id",
        "role"
      ],
      "Permissões": "Necessita de token do dono da conta ou admin",
      "Retornos": {
        "200": "Atualização bem-sucedida",
        "403": "Permissão negada"
      }
    }
  },
  "DICA": "parâmetros marcados com '?' são opcionais"
}
```

## Afazeres

- [ ] Permitir o gerenciamento de funcionários no CRUD de sebos.
- [ ] Elaborar um sistema de permissões melhor.
- [ ] Fazer rotas que dão informações sobre os relacionamentos usuário-sebo,
rota /usersebo
- [ ] Elaborar CRUD para os catálogos e os livros.
