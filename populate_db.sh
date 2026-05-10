curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name": "Allan Melvin", "email": "melvin@ifce.edu.br", "login": "melvin.dtel", "password": "melvin212", "cell_number": "12341234", "is_activated": 12}' \
  http://localhost:3000/users

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name": "Allan Melvin 2", "email": "melvin2@ifce.edu.br", "login": "melvin2.dtel", "password": "melvin212", "cell_number": "12341233", "is_activated": 13}' \
  http://localhost:3000/users

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name": "Sebo do Caba Bom", "cnpj": "sebooo", "street": "Grove", "number": 12, "city": "Los Demiurgos", "state": "Baía de Todos os Hereges", "city_block": "Benfica", "cep": "abublalson", "owners": [1, 2], "workers": []}' \
  http://localhost:3000/stores
