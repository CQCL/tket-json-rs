# List the available commands
help:
    @just --list --justfile {{justfile()}}

# Prepare the environment for development, installing all the dependencies and
# setting up the pre-commit hooks.
setup:
    uv run pre-commit install -t pre-commit

# Run the pre-commit checks.
check:
    HUGR_TEST_SCHEMA=1 uv run pre-commit run --all-files

# Run all the rust tests
test:
    cargo test --all-features

# Auto-fix all clippy warnings
fix:
    cargo clippy --all-targets --all-features --workspace --fix --allow-staged

# Check for missing optypes
check-optypes:
    uv sync
    uv run -- cargo test -- --ignored missing_optypes
