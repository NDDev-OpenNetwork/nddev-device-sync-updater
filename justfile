set shell := ["bash", "-euo", "pipefail", "-c"]

check:
    cargo fmt -- --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked

provider-check:
    python3 scripts/check-provider.py
