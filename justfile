default:
  @just --list

dev:
    @echo "Running dev single-player mode"
    cargo run -- -p 1

release-build:
    cargo build --bin robot-rumble --release --no-default-features --features "embedded_assets discord_presence"

android-build:
    cargo apk build --release --no-default-features

wasm-build:
    @echo "Building WASM release target"
    cargo build --bin robot-rumble --profile wasm-release --target wasm32-unknown-unknown --no-default-features --features embedded_assets
    @echo "In order to build the website template run:"
    @echo "wasm-bindgen --no-typescript --target web --out-dir <OUT_DIR_PATH> --out-name \"robot-rumble\" target/wasm32-unknown-unknown/wasm-release/robot-rumble-client.wasm"
    
check:
    @echo "Checking code formatting (rustfmt)"
    cargo fmt --all --check
    @echo "Checking code formatting (rustfmt)"
    cargo clippy --all-targets --all-features -- -D warnings
    @echo "Running tests"
    cargo test

format:
    @echo "Formatting code"
    cargo fmt
    
