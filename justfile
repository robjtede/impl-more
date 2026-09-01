import '.toolchain/rust.just'

_list:
    @just --list

toolchain := ""

# Check project.
check: clippy
    just --unstable --fmt --check
    nixpkgs-fmt --check .
    cd ./crates/impl-more && cargo rdme --check
    fd --hidden --type=file --extension=md --extension=yml --exec-batch prettier --check
    fd --hidden --extension=toml --exec-batch taplo format --check
    fd --hidden --extension=toml --exec-batch taplo lint
    cargo +nightly fmt -- --check

# Format project.
fmt: update-readmes
    just --unstable --fmt
    nixpkgs-fmt .
    fd --hidden --type=file --extension=md --extension=yml --exec-batch prettier --write
    fd --hidden --extension=toml --exec-batch taplo format
    cargo +nightly fmt

# Update README from crate root documentation.
update-readmes:
    cd ./crates/impl-more && cargo rdme --force

# Lint workspace with Clippy.
clippy toolchain="":
    cargo {{ toolchain }} clippy --workspace --no-default-features
    cargo {{ toolchain }} clippy --workspace --all-features

# Downgrade dependencies required to testing using MSRV.
downgrade-msrv:
    @ echo "No downgrades currently necessary."

# Test workspace.
test: test-no-coverage build-no-std

# Test workspace (without generating coverage output).
test-no-coverage toolchain="":
    cargo {{ toolchain }} nextest run --workspace --all-targets --all-features

# Test docs.
test-docs toolchain="":
    cargo {{ toolchain }} test --doc --workspace --all-features

# Test workspace (without generating coverage output).
build-no-std:
    cargo build --target=thumbv6m-none-eabi --manifest-path=ensure-no-std/Cargo.toml

# Build rustdoc.
doc:
    - rm -f "$(cargo metadata --format-version=1 | jq -r '.target_directory')/doc/crates.js"
    RUSTDOCFLAGS="--cfg=docsrs -Dwarnings" cargo +nightly doc --workspace --no-deps --all-features
