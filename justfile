run:
    cargo run

runr:
    cargo run --release

build:
    cargo build

buildr:
    cargo build --release

clip:
    cargo clippy --fix --allow-dirty --allow-staged --all-targets

fix:
    grep -rlZ --include='*.rs' --include='*.slint' --include='*.md' --include='*.ftl' --exclude='AGENTS.md' --exclude='Justfile' '[─–—]' . | xargs -0 -r sed -i 's/[─–—]/-/g' || true
    cargo +nightly fmt
    cargo clippy --fix --allow-dirty --allow-staged --all-targets
    cargo +nightly fmt
    cargo fmt

upgrade:
    cargo update
