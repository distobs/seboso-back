# Mãozinha do GPT
# O último esboço de script estava uma gambiarra do capeta e não me orgulho
# de tê-lo escrito.

def ask(fields):
    data = {}

    for key, label in fields.items():
        data[key] = input(f'{label}: ').strip()

    return data


def get_command():
    return input('> ').strip().lower()


def show_commands(commands):
    print('\nComandos disponíveis:')

    for command in commands:
        print(f'- {command}')

    print('- exit')


def search_menu(valid_fields):
    campo = input('Campo de pesquisa: ').strip().lower()

    if campo not in valid_fields:
        print('Campo inválido.')
        return

    print(f'Pesquisando por "{campo}"...')
    print('Ainda não implementado.')


def entity_menu(name, actions):
    while True:
        print(f'\n== {name.upper()} ==')

        show_commands(actions.keys())

        command = get_command()

        if command == 'exit':
            print(f'Saindo do menu {name}...')
            break

        action = actions.get(command)

        if action is None:
            print('Comando desconhecido.')
            continue

        action()


# =====================================
# USER
# =====================================

USER_FIELDS = {
    'nome': 'Nome',
    'email': 'Email',
    'login': 'Login',
    'password': 'Senha',
    'cell_number': 'Telefone',
}


def user_list():
    print('Listando usuários...')


def user_create():
    data = ask(USER_FIELDS)

    data['is_activated'] = True

    print('Criando usuário...')
    print(data)


def user_read():
    print('Dica: use a pesquisa para descobrir o ID.')

    user_id = input('ID do usuário: ').strip()

    print(f'Lendo usuário {user_id}...')


def user_update():
    print('Dica: é necessário login.')

    user_id = input('ID do usuário: ').strip()

    data = ask(USER_FIELDS)

    print(f'Atualizando usuário {user_id}...')
    print(data)


def user_delete():
    print('Dica: é necessário login.')

    user_id = input('ID do usuário: ').strip()

    print(f'Deletando usuário {user_id}...')


def user_search():
    search_menu([
        'id',
        'name',
        'email',
        'login',
        'cell_number'
    ])

def user_login():
    login = input('Login: ').strip()
    senha = input('Senha: ').strip()


USER_ACTIONS = {
    'list': user_list,
    'create': user_create,
    'read': user_read,
    'update': user_update,
    'delete': user_delete,
    'search': user_search,
    'login': user_login,
}


# =====================================
# SEBO
# =====================================

SEBO_FIELDS = {
    'nome': 'Nome',
    'cnpj': 'CNPJ',
    'street': 'Rua',
    'number': 'Número',
    'city': 'Cidade',
    'state': 'Estado',
    'city_block': 'Bairro',
    'cep': 'CEP',
}


def sebo_list():
    print('Listando sebos...')


def sebo_create():
    data = ask(SEBO_FIELDS)

    data['funcionarios'] = input(
        'Funcionários ([{id, cargo}]): '
    ).strip()

    print('Criando sebo...')
    print(data)


def sebo_read():
    sebo_id = input('ID do sebo: ').strip()

    print(f'Lendo sebo {sebo_id}...')


def sebo_update():
    sebo_id = input('ID do sebo: ').strip()

    data = ask(SEBO_FIELDS)

    print(f'Atualizando sebo {sebo_id}...')
    print(data)


def sebo_delete():
    sebo_id = input('ID do sebo: ').strip()

    print(f'Deletando sebo {sebo_id}...')


def sebo_search():
    search_menu([
        'id',
        'nome',
        'cnpj',
        'rua',
        'numero',
        'cidade',
        'estado',
        'bairro',
        'cep'
    ])


SEBO_ACTIONS = {
    'list': sebo_list,
    'create': sebo_create,
    'read': sebo_read,
    'update': sebo_update,
    'delete': sebo_delete,
    'search': sebo_search,
}


# =====================================
# BOOK
# =====================================

BOOK_FIELDS = {
    'title': 'Título',
    'description': 'Descrição',
    'launched_at': 'Data de lançamento',
    'cover_type': 'Tipo de capa',
    'author': 'Autor',
    'edition': 'Edição',
    'language': 'Idioma',
    'genre': 'Gênero',
    'isbn_10_code': 'ISBN-10',
    'isbn_13_code': 'ISBN-13',
    'publisher': 'Editora',
    'pages': 'Páginas',
    'dimensions': 'Dimensões',
}


def book_list():
    print('Listando livros...')


def book_create():
    print('Somente admins podem criar livros.')

    data = ask(BOOK_FIELDS)

    print('Criando livro...')
    print(data)


def book_read():
    book_id = input('ID do livro: ').strip()

    print(f'Lendo livro {book_id}...')


def book_update():
    book_id = input('ID do livro: ').strip()

    data = ask(BOOK_FIELDS)

    print(f'Atualizando livro {book_id}...')
    print(data)


def book_delete():
    book_id = input('ID do livro: ').strip()

    print(f'Deletando livro {book_id}...')


def book_search():
    search_menu([
        'id',
        'title',
        'author'
    ])


BOOK_ACTIONS = {
    'list': book_list,
    'create': book_create,
    'read': book_read,
    'update': book_update,
    'delete': book_delete,
    'search': book_search,
}


# =====================================
# MAIN
# =====================================

MAIN_ACTIONS = {
    'user': lambda: entity_menu('user', USER_ACTIONS),
    'sebo': lambda: entity_menu('sebo', SEBO_ACTIONS),
    'book': lambda: entity_menu('book', BOOK_ACTIONS),
}


def help_menu():
    print('\nMenus disponíveis:')
    print('- user')
    print('- sebo')
    print('- book')
    print('- help')
    print('- q')


# =====================================
# LOOP PRINCIPAL
# =====================================

help_menu()

while True:
    command = get_command()

    if command == 'q':
        print('Saindo...')
        break

    if command == 'help':
        help_menu()
        continue

    action = MAIN_ACTIONS.get(command)

    if action is None:
        print('Comando desconhecido.')
        continue

    action()