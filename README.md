# robot-rumble-rs

The Rust rewrite of [Robot Rumble](https://github.com/GaspardCulis/robot-rumble)

## Building

This project uses nightly Rust. In order to minimize compile times, the `mold`
linker and `cranelift` codegen backend are required.

See the
[official Bevy guide](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional)
for more information.

### Linux (Debian based)

```sh
# System dependencies
sudo apt install pkg-config libwayland-dev libasound2-dev libudev-dev
# Compilation dependencies
sudo apt install mold clang
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

### NixOS

Everything is included in the workspace `shell.nix`.

```
nix-shell
```

### Cringedows

```sh
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

## Exporting

This section covers exporting the game to specific platforms using the
[just](https://github.com/casey/just) command runner.

```sh
just --list
# Run some recipe
just release-build
```