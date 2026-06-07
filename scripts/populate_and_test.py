from os import getenv
from dotenv import load_dotenv
import psycopg2
import requests

load_dotenv()

# HELPER
def req(method, rt, desc, jdata=None, jwt = None):
    endpoint = "http://localhost:3000" + rt
    fun = None
    match method:
        case "get":
            fun = requests.get
        case "post":
            fun = requests.post
        case "put":
            fun = requests.put
        case "delete":
            fun = requests.delete

    if jwt:
        r = fun(endpoint, json=jdata, headers={"Authorization": f"Bearer {jwt}"})
    else:
        r = fun(endpoint, json=jdata)

    j = r.json()
    print(j, r.status_code)
    print(desc)
    
    return j

def reqesp(method, rt, desc, jdata=None, jwt = None):
    endpoint = "http://localhost:3000" + rt
    fun = None
    match method:
        case "get":
            fun = requests.get
        case "post":
            fun = requests.post
        case "put":
            fun = requests.put
        case "delete":
            fun = requests.delete

    if jwt:
        r = fun(endpoint, json=jdata, headers={'Authorization': f"Bearer {jwt}"})
    else:
        r = fun(endpoint, json=jdata)
    
    print(r.text)
    print(desc)

# SAVE ADMIN JWT

POSTGRES_DB=getenv("POSTGRES_DB")
POSTGRES_USER=getenv("POSTGRES_USER")
POSTGRES_PASSWORD=getenv("POSTGRES_PASSWORD")

connection = psycopg2.connect(
    database=POSTGRES_DB,
    user=POSTGRES_USER,
    password=POSTGRES_PASSWORD,
    host="localhost",
    port="5432"
)

cursor = connection.cursor()

req("post", "/users", "Create admin", {
    "name": "Admin",
    "email": "admin@admin.com",
    "login": "admin",
    "password": "admin",
    "cell_number": "0000000000"
})

cursor.execute("UPDATE users SET is_admin = true WHERE login = 'admin';")

connection.commit()

j = req("post", "/users/login", "Login as admin", {
    "login": "admin",
    "password": "admin"
})

jwt_adm = j["message"]

# CREATE USERS

req("post", "/users", "Create user John 1", {
    "name": "John 1",
    "email": "john@john.com",
    "login": "john",
    "password": "1234",
    "cell_number": "123412342",
})

req("post", "/users", "Create user John 2", {
    "name": "John 2",
    "email": "john2@john.com",
    "login": "john2",
    "password": "1234",
    "cell_number": "123412343",
})

# READ USERS

req("get", "/users", "List users.")

req("get", "/users/2", "Get User with ID 2")

# LOGIN AND UPDATE USERS

j = req("post", "/users/login", "Login as john2", {
    "login": "john2",
    "password": "1234"
})

jwt = j["message"]

req("put", "/users/3", "Update John 2's name to Joseph 2", jdata={
    "name": "Joseph 2",
}, jwt=jwt)

req("get", "/users/3", "Confirm changes in Joseph 2.")

# DELETE USER

req("delete", "/users/3", "Delete Joseph 2", jwt=jwt)

req("get", "/users", "Confirm deletion")

# LOGIN AS JOHN 1

j = req("post", "/users/login", "Login as john1", {
    "login": "john",
    "password": "1234"
})

jwt = j["message"]

# CREATE STORES

sebos = [
    {
        "name": "Sebo 1",
        "cnpj": "1234",
        "street": "Rua da Goiaba",
        "number": 42,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0001"
    },
    {
        "name": "Sebo 2",
        "cnpj": "1235",
        "street": "Rua da Maçã",
        "number": 43,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0002"
    },
    {
        "name": "Sebo 3",
        "cnpj": "1236",
        "street": "Rua da Maçã",
        "number": 43,
        "city": "Lugarlândia",
        "state": "Localidade",
        "city_block": "ABCD",
        "cep": "0003"
    },
]

for sebo in sebos:
    req("post", "/stores", f"Criando sebo {sebo["name"]}", jdata=sebo, jwt=jwt)

# UPDATE STORES

sebos[0]["name"] = "Seboso 1"
sebos[0]["street"] = "Rua dos Penduricalhos"
sebos[1]["name"] = "Seboso 2123"
sebos[1]["street"] = "Av. Treze de Maio"
sebos[2]["number"] = 1234

for i in range(0, 3):
    req("put", f"/stores/{i + 1}", f"Atualizando sebo {sebos[i]["name"]}", jdata=sebos[i], jwt=jwt)

# READ STORES

req("get", "/stores", "Lendo sebos")

# ADD IN SOME BOOKS

books = [
    {
	"title": "Dom Casmurro",
	"author": "Machado de Assis",
	"description": "Descrição de Dom Casmurro",
	"published_at": "6767-06-07",
	"isbn_10_code": "1234567890",
	"isbn_13_code": "1234567890123",
	"cover_type": "Capa comum",
	"edition": "1",
	"language": "Português",
	"genre": "Realismo",
	"publisher": "Livraria Garnier",
	"pages": 1000,
	"dimensions": "20x20x2 cm"
    },
    {
	"title": "Dom Casmurro 2",
	"author": "Enxada de Assis",
	"description": "Descrição de Dom Casmurro 2",
	"published_at": "2200-12-12",
	"isbn_10_code": "1234567891",
	"isbn_13_code": "1234567890124",
	"cover_type": "Capa super dura",
	"edition": "2",
	"language": "Português 2",
	"genre": "Realismo 2",
	"publisher": "Livraria Garnier 2",
	"pages": 1001,
	"dimensions": "20x20x2 cm"
    },
    {
	"title": "Dom Casmurro 3",
	"author": "Enxada de Assis",
	"description": "Descrição de Dom Casmurro 3",
	"published_at": "2025-01-01",
	"isbn_10_code": "1234567892",
	"isbn_13_code": "1234567890125",
	"cover_type": "Capa ultra dura",
	"edition": "5",
	"language": "Fala-galego-portugo-mirandês (atualizado)",
	"genre": "Ultrarrealismo fictício pós-estruturalista",
	"publisher": "Cartoon Network",
	"pages": 2,
	"dimensions": "20x20x2 km"
    },
]

for book in books:
    req("post", "/books", f"Criando livro {book['title']}", jdata=book, jwt=jwt_adm)

# ADD SOME BOOKS TO STORES' CATALOGS

catalogs = [
    {
        "store_id": 1,
        "book_id": 1,
        "price": 10.99,
        "quantity": 5,
        "description": "só o filé" 
    },
    {
        "store_id": 1,
        "book_id": 2,
        "price": 12.99,
        "quantity": 3,
        "description": "todo lascado"
    },
    {
        "store_id": 2,
        "book_id": 2,
        "price": 11.99,
        "quantity": 4,
        "description": "bom"
    },
    {
        "store_id": 2,
        "book_id": 3,
        "price": 15.99,
        "quantity": 2,
        "description": "ótimo"
    }
]

# John 1 owns everything anyway.
for catalog in catalogs:
    req("post", "/catalog", f"Adicionando livro {catalog['book_id']} ao catálogo da loja {catalog['store_id']}", jdata=catalog, jwt=jwt)

# Create a new guy

req("post", "/users", "Create user John 3", {
    "name": "John 3",
    "email": "john3@john.com",
    "login": "john3",
    "password": "1234",
    "cell_number": "123412343",
})

j = req("post", "/users/login", "Login as John 3", {
    "login": "john3",
    "password": "1234"
})

jwt2 = j["message"]

# Make the new guy a worker

req("post", "/userstore", "Make John 3 a worker in sebo Seboso 1", jdata={
    "store_id": 1,
    "user_id": 4,
    "role": "worker"
}, jwt=jwt)

# Do worker stuff

req("put", "/catalog?store_id=1&book_id=1", "Update book 1 in catalog of Seboso 1 as John 3", jdata={
        "price": 100.50,
        "quantity": 1234,
}, jwt=jwt2)

req("delete", "/catalog?store_id=1&book_id=2", "Delete book 2 from catalog of Seboso 1 as John 3", jwt=jwt2)

# Fire John 3

req("delete", "/userstore", "Fire John 3", jdata={
        "user_id": 4,
        "store_id": 1,
        "role": "worker",
}, jwt=jwt)