# */users*

GET:
/users(id): Informações de um usuário
/users(dados de busca): Pesquisa usuários de acordo com os parâmetros da query. Retorna uma lista de dados com ids

POST:
/users(dados): Cria um usuário com os dados fornecidos

PUT:
/users(dados): Atualiza os dados de um usuário (excluindo o cargo dentro de um sebo. Isso será administrado na rota do sebo

DELETE:
/users(id): Deleta um usuário

# */sebos*

GET:

/store(id): Informações de um store
/store(dados de busca): Query

POST:

/store(dados): Cria. Atrela automaticamente o dono.

PUT:

/store(dados): Atualiza os dados principais
/store/role(dados): Edita donos e funcionários
/store/catalogo(id do livro): Adiciona um livro ao catálogo (antes de usar, usar a rota /catalogo pra add o livro)

DELETE:

/store(id): Deleta um sebo

# */catalogo*

GET:

/catalog: todo o catálogo
/catalog(id_store): catálogo de um sebo específico
/catalog(dados de busca): todo o catálogo

POST:

/catalog(id_store)(dados): inserção do livro no catálogo da loja

PUT:

/catalog(id_store > id_book)(dados): altera os dados do livro em determinada loja

DELETE:

/catalog(id_store > id_book): delete de um livro em alguma loja.

# */books*

GET:

/books: todos os livros
/books(isbn10): acha o livro pelo código ISBN-10

POST:

/books(dados): Upa um livro no banco

PUT:

/books(id)(dados): muda os dados do livro

DELETE:

/books(id): deleta o livro