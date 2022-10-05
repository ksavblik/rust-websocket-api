# Books Rust API with Live Updates Support through the WebSockets

## Required prerequisites

1. [Docker](https://www.docker.com/) or natively installed [PostgreSQL](https://www.postgresql.org/download/)
2. [Rust](https://www.rust-lang.org/tools/install)

## Setup guide

Download or clone the repo:
```sh
git clone https://github.com/ksavblik/rust-websocket-api
```

Open the project in a terminal, install deps and run it:
```sh
cd ./rust-websocket-api
cargo build --release
# run the docker container, skip this cmd if you have active PostgreSQL instance
docker run -d --name rust_postgres -p 5432:5432 --env POSTGRES_USER=postgres --env POSTGRES_PASSWORD=postgres postgres
cargo run --release
```

This is it, feel free to hack the code!😄