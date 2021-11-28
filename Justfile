export RUST_BACKTRACE := "1"
export RUST_LOG := "debug"

clean:
    cargo clean

clean-aurelia:
    cargo clean -p aurelia # rust ICE bug

run:
    cargo run --bin aurelia

test target='':
    cargo test {{target}}

migrate:
    cargo run --bin cli -- migrate

jwt:
    cargo run --bin cli -- create-jwt