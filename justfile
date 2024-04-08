# List the available commands
help:
    @just --list --justfile {{justfile()}}

# Run all the rust tests
test:
    cargo test --all-features

# Auto-fix all clippy warnings
fix:
    cargo clippy --all-targets --all-features --workspace --fix --allow-staged

# Check for missing optypes
check-optypes:
    poetry -C tests update
    poetry -C tests run -- cargo test -- --ignored missing_optypes