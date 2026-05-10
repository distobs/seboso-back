#!/bin/bash

# Feito por IA

API="http://localhost:3000"

echo "Creating users..."

for i in {1..10}
do
  curl -s -X POST "$API/users" \
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

LOGIN_RESPONSE=$(curl -s -X POST "$API/users/login" \
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
echo "Done."
