_list:
    @just --list

clippy:
    cargo clippy --workspace --no-default-features
    cargo clippy --workspace --all-features

test:
    cargo test --workspace --all-features
    cargo build --target=thumbv6m-none-eabi --manifest-path=./ensure-no-std/Cargo.toml

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

doc:
    rm "$(cargo metadata --format-version=1 | jq -r '.target_directory')/doc/crates.js"
    RUSTDOCFLAGS="--cfg=docsrs -Dwarnings" cargo +nightly doc --workspace --no-deps --all-features
