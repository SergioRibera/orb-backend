set dotenv-path := ".env"

default:
    cargo watch -x run

preview:
    cargo run --release

c:
    cargo clippy --all

f:
    cargo fmt --all

cm name:
    sqlx migrate add {{ name }}

m:
    sqlx migrate run

cs name:
    grow new {{ name }}

s:
    grow run --all

seed-sql:
    psql $DATABASE_URL -f seeders/seed_sql.sql

prepare:
    cargo install cargo-watch
    cargo install sqlx-cli
    cargo install grow-rs --no-default-features --features "sqlx fake" --force

