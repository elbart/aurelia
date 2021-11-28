export RUST_BACKTRACE := env_var_or_default("RUST_BACKTRACE", "0")
export RUST_LOG := "debug"

default:
    @just --list

clean:
    cargo clean

clean-ice:
    cargo clean -p aurelia # rust ICE bug

run:
    cargo run --bin aurelia

run-bt:
    RUST_BACKTRACE=1 cargo run --bin aurelia

test target='':
    cargo test {{target}}

test-bt target='':
    RUST_BACKTRACE=1 cargo test {{target}}

migrate:
    cargo run --bin cli -- migrate

jwt:
    cargo run --bin cli -- create-jwt