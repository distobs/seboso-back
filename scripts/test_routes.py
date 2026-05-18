import requests
import json

API = "http://localhost:3000"

TOKEN = None


# =========================================================
# UTILS
# =========================================================

def pretty(response):
    print(f"\n[{response.status_code}]")

    try:
        print(json.dumps(response.json(), indent=4, ensure_ascii=False))
    except Exception:
        print(response.text)


def auth_headers():
    if TOKEN is None:
        return {}

    return {
        "Authorization": f"Bearer {TOKEN}"
    }


# =========================================================
# USERS
# =========================================================

def create_user(index):
    print(f"\nCreating user {index}...")

    payload = {
        "name": f"User {index}",
        "email": f"user{index}@example.com",
        "login": f"user{index}",
        "password": "123456",
        "cell_number": f"85999999{index}"
    }

    response = requests.post(
        f"{API}/users",
        json=payload
    )

    pretty(response)


def login_user():
    global TOKEN

    print("\nLogging in...")

    payload = {
        "login": "user1",
        "password": "123456"
    }

    response = requests.post(
        f"{API}/users/login",
        json=payload
    )

    pretty(response)

    try:
        data = response.json()

        # ajusta conforme tua API
        TOKEN = data.get("message")

    except Exception:
        TOKEN = None

    print("\nTOKEN:")
    print(TOKEN)


def list_users():
    print("\nListing users...")

    response = requests.get(
        f"{API}/users?page=1&per_page=10"
    )

    pretty(response)


def get_user(user_id):
    print(f"\nGetting user {user_id}...")

    response = requests.get(
        f"{API}/users/{user_id}"
    )

    pretty(response)


def update_user(user_id):
    print(f"\nUpdating user {user_id}...")

    payload = {
        "name": "Updated User",
        "cell_number": "85888888888"
    }

    response = requests.put(
        f"{API}/users/{user_id}",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


# =========================================================
# STORES
# =========================================================

def create_store(index):
    print(f"\nCreating store {index}...")

    payload = {
        "name": f"Sebo {index}",
        "cnpj": f"1234567890000{index}",
        "street": f"Rua {index}",
        "number": index,
        "city": "Fortaleza",
        "state": "CE",
        "city_block": "Centro",
        "cep": f"6000000{index}"
    }

    response = requests.post(
        f"{API}/stores",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


def list_stores():
    print("\nListing stores...")

    response = requests.get(
        f"{API}/stores?page=1&per_page=10"
    )

    pretty(response)


def get_store(store_id):
    print(f"\nGetting store {store_id}...")

    response = requests.get(
        f"{API}/stores/{store_id}"
    )

    pretty(response)


def update_store(store_id):
    print(f"\nUpdating store {store_id}...")

    payload = {
        "name": "Novo Nome Sebo"
    }

    response = requests.put(
        f"{API}/stores/{store_id}",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


# =========================================================
# BOOKS
# =========================================================

def create_book(index):
    print(f"\nCreating book {index}...")

    payload = {
        "title": f"Livro {index}",
        "author": f"Autor {index}",
        "isbn_10_code": f"123456789{index}",
        "description": f"Descrição {index}",
        "publisher": f"Editora {index}",
        "pages": 100 + index
    }

    response = requests.post(
        f"{API}/books",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


def list_books():
    print("\nListing books...")

    response = requests.get(
        f"{API}/books?page=1&per_page=10"
    )

    pretty(response)


def get_book(isbn):
    print(f"\nGetting book {isbn}...")

    response = requests.get(
        f"{API}/books/{isbn}"
    )

    pretty(response)


def update_book(book_id):
    print(f"\nUpdating book {book_id}...")

    payload = {
        "title": "Livro Atualizado"
    }

    response = requests.put(
        f"{API}/books/{book_id}",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


# =========================================================
# CATALOG
# =========================================================

def create_catalog(store_id, book_id):
    print(f"\nCreating catalog entry store={store_id} book={book_id}...")

    payload = {
        "store_id": store_id,
        "book_id": book_id,
        "price": 19.99,
        "quantity": 10,
        "state": "new"
    }

    response = requests.post(
        f"{API}/catalog",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


def list_catalog():
    print("\nListing catalog...")

    response = requests.get(
        f"{API}/catalog"
    )

    pretty(response)


def get_store_catalog(store_id):
    print(f"\nGetting catalog for store {store_id}...")

    response = requests.get(
        f"{API}/catalog/{store_id}"
    )

    pretty(response)


def update_catalog(store_id, book_id):
    print(f"\nUpdating catalog entry...")

    payload = {
        "store_id": store_id,
        "book_id": book_id,
        "price": 29.99,
        "quantity": 20
    }

    response = requests.put(
        f"{API}/catalog",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


def delete_catalog(store_id, book_id):
    print(f"\nDeleting catalog entry...")

    payload = {
        "store_id": store_id,
        "book_id": book_id
    }

    response = requests.delete(
        f"{API}/catalog",
        json=payload,
        headers=auth_headers()
    )

    pretty(response)


# =========================================================
# MAIN TEST FLOW
# =========================================================

if __name__ == "__main__":

    print("========== USERS ==========")

    for i in range(1, 4):
        create_user(i)

    login_user()

    list_users()
    get_user(1)
    update_user(1)

    print("\n========== STORES ==========")

    for i in range(1, 3):
        create_store(i)

    list_stores()
    get_store(1)
    update_store(1)

    print("\n========== BOOKS ==========")

    for i in range(1, 3):
        create_book(i)

    list_books()
    get_book("1234567891")
    update_book(1)

    print("\n========== CATALOG ==========")

    create_catalog(1, 1)

    list_catalog()
    get_store_catalog(1)

    update_catalog(1, 1)

    delete_catalog(1, 1)

    print("\nDone.")
