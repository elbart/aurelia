export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "0")
export RUST_LOG := "debug"

default:
    @just --list

build:
    cd api && cargo build

clean:
    cd api && cargo clean

# seems deprecated since rust 1.57.0
clean-ice:
    cd api && cargo clean -p aurelia # rust ICE bug

run-api:
    cd api && cargo watch -x 'run --bin aurelia'

run-bt:
    cd api && RUST_BACKTRACE=1 cargo watch -x 'run --bin aurelia'

run-frontend:
    cd frontend && npm run dev

run-database:
    docker compose up -d && docker compose logs -f

test target='':
    cd api && cargo test {{target}} -- --nocapture

test-bt target='':
    cd api && RUST_BACKTRACE=1 cargo test {{target}}

migrate:
    cd api && cargo run --bin cli -- migrate

jwt:
    cd api && cargo run --bin cli -- create-jwt

