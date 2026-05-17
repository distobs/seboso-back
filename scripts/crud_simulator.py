def user():
    print('"list" para listar usuários')
    print('"create" para criar usuário')
    print('"read" para ler usuário')
    print('"update" para atualizar usuário')
    print('"delete" para deletar usuário')
    print('"login" para fazer login')

    comando = input('> ').lower().strip().split()[0]

    if comando == 'list':
        # Request para listar usuários
        pass
    elif comando == 'create':
        nome = input('Nome: ')
        email = input('Email: ')
        login = input('Login: ')
        password = input('Senha: ')
        cell_number = input('Telefone: ')
        is_activated = 1234 # Tem que refatorar essa desgraça
    elif comando == 'read':
        print("Dica: use a pesquisa para descobrir o ID do usuário")
        user_id = input('ID do usuário: ')
    elif comando == 'update':
        print("Dica: use a pesquisa para descobrir o ID do usuário")
        print("Dica: é necessário estar logado")
        user_id = input('ID do usuário: ')
        nome = input('Nome: ')
        email = input('Email: ')
        login = input('Login: ')
        password = input('Senha: ')
        cell_number = input('Telefone: ')
        is_activated = 1234 # Tem que refatorar essa desgraça
    elif comando == 'delete':
        print("Dica: use a pesquisa para descobrir o ID do usuário")
        print("Dica: é necessário estar logado")
        user_id = input('ID do usuário: ')
        print('Deleting user...')
    elif comando == 'search':
        campo = input('Campo de pesquisa (id, name, email, login, cell_number): ')
        
        if campo not in ['id', 'name', 'email', 'login', 'cell_number']:
            print('Campo de pesquisa inválido.')
        else:
            print('Ainda vamos implementar isso.')
    elif comando == 'login':
        login = input('Login: ')
        senha = input('Senha: ')
    else:
        print('Comando desconhecido.')

def sebo():
    print('"list" para listar sebos')
    print('"create" para criar sebo')
    print('"read" para ler sebo')
    print('"update" para atualizar sebo')
    print('"delete" para deletar sebo')

    comando = input('> ').lower().strip().split()[0]

    if comando == 'list':
        # Request para listar sebos
        pass
    elif comando == 'create':
        nome = input('Nome: ')
        cnpj = input('Email: ')
        street = input('Rua: ')
        number = input('Número: ')
        city = input('Cidade: ')
        state = input('Estado: ')
        city_block = input('Bairro: ')
        cep = input('CEP: ')
        funcionarios = input('Funcionários (seguir formato: [\{id1, cargo\}, \{id2, cargo\}, ...]): ')
    elif comando == 'read':
        print("Dica: use a pesquisa para descobrir o ID do sebo")
        sebo_id = input('ID do sebo: ')
    elif comando == 'update':
        print("Dica: use a pesquisa para descobrir o ID do sebo")
        print("Dica: é necessário estar logado como um dos funcionários do sebo")
        sebo_id = input('ID do sebo: ')
        nome = input('Nome: ')
        cnpj = input('CNPJ: ')
        street = input('Rua: ')
        number = input('Número: ')
        city = input('Cidade: ')
        state = input('Estado: ')
        city_block = input('Bairro: ')
        cep = input('CEP: ')
    elif comando == 'delete':
        print("Dica: use a pesquisa para descobrir o ID do sebo")
        print("Dica: é necessário estar logado como um dos funcionários do sebo")
        sebo_id = input('ID do sebo: ')
    elif comando == 'search':
        campo = input('Campo de pesquisa (id, nome, cnpj, rua, numero (sem acento), cidade, estado, bairro, cep): ')
        
        if campo not in ['id', 'nome', 'cnpj', 'rua', 'numero', 'cidade', 'estado', 'bairro', 'cep']:
            print('Campo de pesquisa inválido.')
        else:
            print('Ainda vamos implementar isso.')
    else:
        print('Comando desconhecido.')

def book():
    print('"list" para listar livros')
    print('"create" para criar livro')
    print('"read" para ler livro')
    print('"update" para atualizar livro')
    print('"delete" para deletar livro')
    print('"search" para pesquisar livros')

    comando = input('> ').lower().strip().split()[0]
    """
        CREATE TABLE IF NOT EXISTS "books" (
    	"id" bigserial NOT NULL UNIQUE,
    	"title" varchar(255) NOT NULL,
    	"description" varchar(255) NOT NULL,
    	"launched_at" varchar(255) NOT NULL,
    	"cover_type" varchar(255) NOT NULL,
    	"author" varchar(255) NOT NULL,
    	"edition" varchar(255) NOT NULL,
    	"language" varchar(255) NOT NULL,
    	"genre" varchar(255) NOT NULL,
    	"isbn_10_code" bigint NOT NULL,
    	"isbn_13_code" varchar(255),
    	"publisher" varchar(255) NOT NULL,
    	"pages" bigint NOT NULL,
    	"dimentions" varchar(255) NOT NULL,
    	"created_at" timestamp with time zone NOT NULL DEFAULT NOW(),
    	"updated_at" timestamp with time zone NOT NULL DEFAULT NOW(),
    	PRIMARY KEY ("id")
    );
    """

    if comando == 'list':
        # Request para listar livros
        pass
    elif comando == 'create':
        title = input('Título : ')
        description = input('Descrição: ')
        launched_at = input('Data de lançamento: ')
        cover_type = input('Tipo de capa: ')
        author = input('Autor: ')
        edition = input('Edição: ')
        language = input('Idioma: ')
        genre = input('Gênero: ')
        isbn_10_code = input('Código ISBN-10: ')
        isbn_13_code = input('Código ISBN-13: ')
        publisher = input('Editora: ')
        pages = input('Páginas: ')
        dimentions = input('Dimensões: ')
    elif comando == 'read':
        print("Dica: use a pesquisa para descobrir o ID do sebo")
        sebo_id = input('ID do book: ')
    elif comando == 'update':
        print("Dica: use a pesquisa para descobrir o ID do livro")
        print("Dica: é necessário estar logado como admin ou como pessoa que adicionou o livro")
        title = input('Título : ')
        description = input('Descrição: ')
        launched_at = input('Data de lançamento: ')
        cover_type = input('Tipo de capa: ')
        author = input('Autor: ')
        edition = input('Edição: ')
        language = input('Idioma: ')
        genre = input('Gênero: ')
        isbn_10_code = input('Código ISBN-10: ')
        isbn_13_code = input('Código ISBN-13: ')
        publisher = input('Editora: ')
        pages = input('Páginas: ')
        dimentions = input('Dimensões: ')
    elif comando == 'delete':
        print("Dica: use a pesquisa para descobrir o ID do livro")
        print("Dica: é necessário estar logado como admin ou como pessoa que adicionou o livro")
        book_id = input('ID do livro: ')
    elif comando == 'search':
        campo = input('Campo de pesquisa (id, title, description, author, edition, language, genre, isbn_10_code, isbn_13_code, publisher, pages, dimentions, launched_at): ')
        
        if campo not in [id, title, description, author, edition, language, genre, isbn_10_code, isbn_13_code, publisher, pages, dimentions, launched_at]:
            print('Campo de pesquisa inválido.')
        else:
            print('Ainda vamos implementar isso.')
    else:
        print('Comando desconhecido.')

# MAIN LOOP

def help():
    print('Comandos disponíveis:')
    print('"user" para operações em usuários')
    print('"sebo" para operações em sebos')
    print('"book" para operações em livros')
    print('"userstore" para operações em funcionários de sebos')
    print('"catalog" para operações em catálogos de sebos')
    print('"q" para sair')

comando = input('> ').lower().strip().split()[0]

help()

while comando != 'q':
    if comando == 'user':
        user()
    elif comando == 'sebo':
        sebo()
    elif comando == 'book':
        #book()
        pass
    elif comando == 'userstore':
        print('Operações em funcionários de sebos')
    elif comando == 'catalog':
        print('Operações em catálogos de sebos')
    else:
        print('Unknown command.')

    comando = input('> ').lower().strip().split()[0]