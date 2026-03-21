# */user*

GET:
/user(id): Informações de um usuário
/user(dados de busca): Pesquisa usuários de acordo com os parâmetros da query. Retorna uma lista de dados com ids

POST:
/user(dados): Cria um usuário com os dados fornecidos

PUT:
/user(dados): Atualiza os dados de um usuário (excluindo o cargo dentro de um sebo. Isso será administrado na rota do sebo

DELETE:
/user(id): Deleta um usuário

# */sebo*

GET:

/sebo(id): Informações de um sebo
/sebo(dados de busca): Query

POST:

/sebo(dados): Cria. Atrela automaticamente o dono.

PUT:

/sebo(dados): Atualiza os dados principais
/sebo/role(dados): Edita donos e funcionários
/sebo/catalogo(id do livro): Adiciona um livro ao catálogo (antes de usar, usar a rota /catalogo pra add o livro)

DELETE:

/sebo(id): Deleta um sebo

# */catalogo*

_mais ou menos análogo_
