FROM rust:1.94

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY .env ./

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build

COPY src ./src
RUN cargo build

CMD ["cargo", "run"]