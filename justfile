default:
  @just --list

dev:
    @echo "Running dev single-player mode"
    cargo run -- -p 1

release-build:
    cargo build --release --no-default-features --features embedded_assets

wasm-build:
    @echo "Building WASM release target"
    cargo build --profile wasm-release --target wasm32-unknown-unknown --no-default-features --features embedded_assets
    @echo "Generating website"
    wasm-bindgen --no-typescript --target web --out-dir ./web --out-name "robot-rumble" target/wasm32-unknown-unknown/wasm-release/robot-rumble-client.wasm
    
