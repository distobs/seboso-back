# Back-end do Seboso

## O que é?

O Seboso quer fornecer uma interface prática para a descoberta e o gerenciamento
de sebos, visando ampliar a cultura do sebo. Este é o back-end - o servidor - do
Seboso.

## Como rodar?

No Linux:

- Clone o repositório:

```bash
$ git clone https://github.com/distobs/seboso-back.git
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
```

No Windows: deve ser parecido, mas não tenho um Windows instalado pra testar.
 O documento está aberto para melhor detalhamento.

## Como testar

No Linux:

```bash
$ chmod +x populate-db.bash
$ ./populate-db.bash
```

No Windows: WSL, Cygwin e adjacentes devem resolver. Fazer os requests manualmente também.

## Documentação das rotas criadas até o momento em que o JSON foi atualizado.

- Nota para o front-end: é bem provável que vocês acabem descobrindo alguns
erros com nosso back-end.

```json
{
  "/stores": {
    "DELETE /sebos/{sebo_id}": "Exclui sebo, necessita de token de um funcionário com role 'worker' ou 'owner'",
    "GET /sebos/{sebos_id}": {
      "Descrição": "Retorna informações de um sebo com base no ID"
    },
    "GET /stores?page=<>&per_page=<>": {
      "Descrição": "Lista sebos, com paginação",
      "Parâmetros (de URL)": {
        "page": "Página",
        "per_page": "Número de sebos por página"
      }
    },
    "POST /sebos/": {
      "Adicional: parâmetro workers": {
        "Descrição": "O parâmetro 'workers' deve ser uma lista de objetos com o formato descrito abaixo",
        "Formato": {
          "role": "Cargo do funcionário dentro do sebo.",
          "user_id": "ID do usuário a ser adicionado como funcionário do sebo"
        }
      },
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
        "workers"
      ]
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
        "cep"
      ]
    }
  },
  "/users": {
    "DELETE /users/{user_id}": {
      "Descrição": "Exclui usuário, necessita de token do dono da conta"
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
        "cell_number",
        "is_activated"
      ]
    },
    "POST /users/login": {
      "Descrição": "Faz login e retorna token JWT",
      "Parâmetros (JSON)": [
        "login",
        "password"
      ]
    },
    "PUT /users/{user_id}": {
      "Descrição": "Atualiza um usuário, necessita de token do dono da conta",
      "Parâmetros (JSON)": [
        "name",
        "email",
        "login",
        "password",
        "cell_number",
        "is_activated"
      ]
    }
  }
}
```

## Afazeres

- [ ] Permitir o gerenciamento de funcionários no CRUD de sebos.
- [ ] Elaborar um sistema de permissões melhor.
- [ ] Fazer rotas que dão informações sobre os relacionamentos usuário-sebo,
rota /usersebo
- [ ] Elaborar CRUD para os catálogos e os livros.