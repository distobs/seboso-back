#!/bin/bash

# Feito por IA

API="http://localhost:3000"

echo "Creating users..."

for i in {1..10}
do
  curl -X POST "$API/users" \
    -H "Content-Type: application/json" \
    -d "{
      \"name\": \"User $i\",
      \"email\": \"user$i@example.com\",
      \"login\": \"user$i\",
      \"password\": \"123456\",
      \"cell_number\": \"85999999$i\",
      \"is_activated\": 1
    }"

  echo ""
done

echo ""
echo "Logging in as user1..."

LOGIN_RESPONSE=$(curl -X POST "$API/users/login" \
  -H "Content-Type: application/json" \
  -d '{
    "login": "user1",
    "password": "123456"
  }')

TOKEN=$(echo "$LOGIN_RESPONSE" | sed -n 's/.*"message":"\([^"]*\)".*/\1/p')

echo "JWT:"
echo "$TOKEN"

echo ""
echo "Creating stores..."

for i in {1..5}
do
  curl -s -X POST "$API/stores" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "{
      \"name\": \"Sebo $i\",
      \"cnpj\": \"1234567890000$i\",
      \"street\": \"Rua $i\",
      \"number\": $i,
      \"city\": \"Fortaleza\",
      \"state\": \"CE\",
      \"city_block\": \"Centro\",
      \"cep\": \"6000000$i\",
      \"workers\": [
        {
          \"user_id\": 1,
          \"role\": \"owner\"
        },
        {
          \"user_id\": 2,
          \"role\": \"worker\"
        }
      ]
    }"

  echo ""
done

echo ""
echo "Creating books..."

for i in {1..5}
do
  curl -s -X POST "$API/books" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "{
      \"title\": \"Livro $i\",
      \"description\": \"Descrição do livro $i\",
      \"launched_at\": \"0$i/01/2024\",
      \"cover_type\": \"Capa comum\",
      \"author\": \"Autor $i\",
      \"edition\": \"${i}ª edição\",
      \"language\": \"Português\",
      \"genre\": \"Ficção\",
      \"isbn_10_code\": 123456789$i,
      \"isbn_13_code\": \"978123456789$i\",
      \"publisher\": \"Editora $i\",
      \"pages\": $((100 + i * 10)),
      \"dimentions\": \"23 x 16 x 2 cm\"
    }"

  echo ""
done

echo ""
echo "Creating books in catalog..."

for i in {1..5}
do
  curl -s -X POST "$API/catalog" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer $TOKEN" \
    -d "{
      \"id_store\": $i,
      \"isbn_10_code_book\": 123456789$i,
      \"price\": 1$i.00,
      \"quantity\": 1$i,
      \"description\": \"Teste teste teste.\"
    }"

  echo ""
done

echo ""
echo "Done."
