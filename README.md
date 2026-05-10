# Back-end do Seboso

## O que é?

O Seboso quer fornecer uma interface prática para a descoberta e o gerenciamento
de sebos, visando ampliar a cultura do sebo. Este é o back-end - o servidor - do
Seboso.

## Como rodar?

No Linux:

- Clone o repositório:

```bash
$ git clone https://github.com/distobs/seboso-back.git
```

- Crie um .env:

```bash
$ cp .env.example .env
$ # abra o .env com seu editor favorito e preencha com os valores que quiser
```

- Instale o Docker e verifique se o comando `docker compose version` funciona.

- Rode e tome café enquanto compila :D

```bash
$ docker compose up --build # Para compilar e iniciar. d (de detach) para deixar em segundo plano.
$ docker compose down -v # para parar o container
$ # às vezes é necessário rodar como root. Se der erro, tente isso.
```

No Windows: deve ser parecido, mas não tenho um Windows instalado pra testar.
 O documento está aberto para melhor detalhamento.

## Como testar

No Linux:

```bash
$ chmod +x populate-db.bash
$ ./populate-db.bash
```

No Windows: WSL, Cygwin e adjacentes devem resolver. Fazer os requests manualmente também.
