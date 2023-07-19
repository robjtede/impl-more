_list:
    @just --list

clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --no-default-features --all-features

test:
    cargo nextest run --workspace --all-targets --all-features
    cargo test --workspace --doc --all-features
    RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --no-deps
    cargo build --target thumbv6m-none-eabi --manifest-path=./ensure-no-std/Cargo.toml

check:
    just --unstable --fmt --check
    npx -y prettier --check '**/*.md'
    taplo lint
    cargo +nightly fmt -- --check

fmt:
    just --unstable --fmt
    npx -y prettier --write '**/*.md'
    taplo format
    cargo +nightly fmt
