#!/bin/bash

set -xe

RUST_LOG=debug cargo run --bin cli -- migrate

# TODO: update when PR #237 is merged in https://github.com/SeaQL/sea-orm/pull/237
#sea-orm-cli generate entity --database-url "postgres://realizor:realizor@localhost:5432/realizor" --output-dir ./src/database/entity --tables tag,ingredient,ingredient_tag,recipe,recipe,recipe_ingredient
RUST_LOG=info /Users/tim/work/sea-orm/sea-orm-cli/target/debug/sea-orm-cli generate entity --database-url "postgres://realizor:realizor@localhost:5432/realizor" --output-dir ./src/database/entity --tables tag,ingredient,ingredient_tag,recipe,recipe,recipe_ingredient,user --with-serde both
