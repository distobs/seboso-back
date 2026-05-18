import requests
from pprint import pprint

API = "http://localhost:3000"

def show_response(response):
    print(f"Status: {response.status_code}")

    try:
        pprint(response.json(), sort_dicts=False)
    except Exception:
        print(response.text)

    print()


print("Creating users...\n")

for i in range(1, 11):
    response = requests.post(
        f"{API}/users",
        json={
            "name": f"User {i}",
            "email": f"user{i}@example.com",
            "login": f"user{i}",
            "password": "123456",
            "cell_number": f"85999999{i}",
            "is_activated": True,
        },
    )

    show_response(response)

print("Logging in as user1...\n")

login_response = requests.post(
    f"{API}/users/login",
    json={
        "login": "user1",
        "password": "123456",
    },
)

show_response(login_response)

body = login_response.json()

# Ajuste isso dependendo do retorno da API
token = body.get("message")

print("JWT:")
print(token)
print()

headers = {
    "Authorization": f"Bearer {token}",
}

print("Creating stores...\n")

for i in range(1, 6):
    response = requests.post(
        f"{API}/stores",
        headers=headers,
        json={
            "name": f"Sebo {i}",
            "cnpj": f"1234567890000{i}",
            "street": f"Rua {i}",
            "number": i,
            "city": "Fortaleza",
            "state": "CE",
            "city_block": "Centro",
            "cep": f"6000000{i}",
        },
    )

    show_response(response)

print("Creating books...\n")

for i in range(1, 6):
    response = requests.post(
        f"{API}/books",
        headers=headers,
        json={
            "title": f"Livro {i}",
            "description": f"Descrição do livro {i}",
            "published_at": f"0{i}/01/2024",
            "cover_type": "Capa comum",
            "author": f"Autor {i}",
            "edition": f"{i}ª edição",
            "language": "Português",
            "genre": "Ficção",
            "isbn_10_code": f"123456789{i}",
            "isbn_13_code": f"978123456789{i}",
            "publisher": f"Editora {i}",
            "pages": 100 + (i * 10),
            "dimensions": "23 x 16 x 2 cm",
        },
    )

    show_response(response)

print("Creating books in catalog...\n")

for i in range(1, 6):
    response = requests.post(
        f"{API}/catalog",
        headers=headers,
        json={
            "store_id": i,
            "isbn_10_code_book": f"123456789{i}",
            "price": float(f"1{i}.00"),
            "quantity": int(f"1{i}"),
            "description": "Teste teste teste.",
        },
    )

    show_response(response)

print("Done.")
