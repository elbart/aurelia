export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "0")
export RUST_LOG := "debug"

default:
    @just --list

build:
    cargo build

clean:
    cargo clean

run-database:
    docker compose up -d && docker compose logs -f

test target='':
    cargo test {{target}} -- --nocapture

test-bt target='':
    RUST_BACKTRACE=1 cargo test {{target}}
