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

prepare:
    cargo install cargo-watch
    cargo install sqlx-cli
