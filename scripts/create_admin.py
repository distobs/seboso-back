import psycopg2
import os
from dotenv import load_dotenv

load_dotenv()
conn = psycopg2.connect(
        host='localhost',
        port=5432,
        database=os.getenv('POSTGRES_DB'),
        user=os.getenv('POSTGRES_USER'),
        password=os.getenv('DB_PASSWORD'))

nome = input('Nome: ').strip()
email = input('Email: ').strip()
login = input('Login: ').strip()
senha = input('Senha: ').strip()

cursor = conn.cursor()

cursor.execute(
        "INSERT INTO users (name, email, login, password, is_admin) VALUES (%s, %s, %s, %s, %s)",
        (nome, email, login, senha, True)
)

print('Foi sal pai')