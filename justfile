build_all:
    cargo build --release
    cargo build
    cargo clippy
    cargo test

upgrade:
    cargo +nightly -Z unstable-options update --breaking
    cargo update

fix:
    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt
    cargo fmt

fixn:
    cargo +nightly fmt
    RUSTUP_TOOLCHAIN=nightly cargo clippy --fix --allow-dirty --allow-staged --all-features --all-targets
    cargo +nightly fmt
    cargo fmt