Entidades principais:

*Usuário*
* ID
* Email
* Login
* Telefone
* Outros dados
* Hash da senha
* Endereço (composto)
* Relacionamento many-to-many: um usuário pode ser dono de um ou mais sebos, um sebo pode ser de um ou mais usuários
* Relacionamento many-to-many: um usuário pode ser funcionário de um ou mais sebos, um sebo pode ter um ou mais funcionários

*Livro/CD/DVD*
* ID
* Dados específicos
* Preço
* Relacionamento many to many (um livro pode estar no catálogo de 1 ou mais sebos, um ou mais sebos podem ter um livro em seu catálogo)

*Sebo*
* ID
* Dados específicos (CNPJ, etc.)
* Descrição
* Localização
* (relacionamento) funcionários
* (relacionamento) donos
* (relacionamento) catálogo