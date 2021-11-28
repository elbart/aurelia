export RUST_BACKTRACE := "1"
export RUST_LOG := "debug"

run:
    cargo clean -p aurelia # rust ICE bug
    cargo run --bin aurelia

test target='':
    cargo clean -p aurelia # rust ICE bug
    cargo test {{target}}

migrate:
    cargo clean -p aurelia # rust ICE bug
    cargo run --bin cli -- migrate

jwt:
    cargo run --bin cli -- create-jwt