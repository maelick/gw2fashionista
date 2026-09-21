set dotenv-load
set lazy

migration_dir := quote(justfile_dir() / "storage/migrations")
crates_dir := `basename -a $(dirname $(cargo metadata --no-deps --format-version 1 | jq .packages[].manifest_path -r)) | tr '\n' ' '`
crates := `cargo metadata --no-deps --format-version 1 | jq .packages[].name -r | tr '\n' ' '`

# Show the list of commands
help:
    @just --list

# Run cargo fmt, check and clippy
lint: format check clippy

# Run cargo check
check:
    cargo check

# Run cargo clippy
clippy:
    cargo clippy

# Run cargo fmt
format:
    cargo fmt

# Run all tests in the workspace
test-all:
    cargo test --all-features

# Run all tests in the workspace
test-e2e:
    cargo test -p e2e --all-features

# Run a specific test on a single thread with info logs and no captured output
test-single test_name:
    RUST_LOG=info cargo test --test {{ quote(test_name) }} -- --nocapture --test-threads=1

# Create the sqlx database
db-create:
    cargo sqlx database create

# Drop the sqlx database without asking for confirmation
db-drop:
    cargo sqlx database drop -y

# Create and migrate the sqlx database
db-migrate: db-create
    cargo sqlx migrate run --source {{ migration_dir }}

# Drop and recreate the sqlx database
db-recreate: db-drop db-migrate

# Prepare sqlx queries for CI (must run after changing checked queries)
sqlx-prepare:
    cargo sqlx prepare --workspace -- --all-targets

# Output the list of crates directories included in the workspace
ls-crates:
    @echo {{ crates_dir }}

# Run cloc on all the crates of the workspace
cloc:
    cloc {{ crates_dir }} --by-file-by-lang
