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
sudo apt-get install mold clang
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

### WASM

This section covers exporting the release version to WASM on a NixOS machine.

```sh
cargo build --profile wasm-release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web --out-dir <OUT_DIR> --out-name "robot-rumble" target/wasm32-unknown-unknown/wasm-release/robot-rumble-client.wasm
```

## TODOS

- \[ \] Player:
  - \[x\] Sprites and animations
  - \[ \] Keep angular velocity on lost
- \[ \] Weapons:
  - \[ \] Shotgun
  - \[ \] Uzi
  - \[ \] Black hole launcher
  - \[ \] Rocket launcher
  - \[ \] Lazer gun
- \[ \] Planets and environment:
  - \[x\] Implement
    [pixel planet](https://deep-fold.itch.io/pixel-planet-generator) shader
  - \[x\] Add background shader using
    [pixel space background](https://deep-fold.itch.io/space-background-generator)
    shader
  - \[ \] Add multiple kinds of planets
    - \[ \] Implement all shader types
      - \[ \]
        [Asteroids](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Asteroids/Asteroids.gdshader)
      - \[x\] BlackHole
        - \[x\]
          [BlackHole](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/BlackHole/BlackHole.gdshader)
        - \[x\]
          [BlackHoleRing](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/BlackHole/BlackHoleRing.gdshader)
      - \[x\]
        [DryTerrain](https://github.com/Deep-Fold/PixelPlanets/tree/main/Planets/DryTerran)
        ?
      - \[ \]
        [Galaxy](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Galaxy/Galaxy.gdshader)
      - \[ \]
        [GasPlanet](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanet/GasPlanet.gdshader)
      - \[x\] SaturnLike
        - \[x\]
          [GasLayers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanetLayers/GasLayers.gdshader)
        - \[x\]
          [Ring](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanetLayers/Ring.gdshader)
      - \[x\]
        [IceWorld](https://github.com/Deep-Fold/PixelPlanets/tree/main/Planets/IceWorld)
        ?
      - \[x\]
        [Clouds](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/Clouds.gdshader)
      - \[x\]
        [PlanetLandmass](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/PlanetLandmass.gdshader)
      - \[x\]
        [PlanetUnder](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/PlanetUnder.gdshader)
      - \[ \]
        [Rivers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LavaWorld/Rivers.gdshader)
      - \[x\]
        [Craters](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/NoAtmosphere/Craters.gdshader)
      - \[x\]
        [NoAtmosphere](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/NoAtmosphere/NoAtmosphere.gdshader)
      - \[ \]
        [LandRivers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Rivers/LandRivers.gdshader)
      - \[x\] Star (FIX: Loop StarFlares and StarBlobs)
        - \[x\]
          [Star](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/Star.gdshader)
        - \[x\]
          [StarBlobs](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/StarBlobs.gdshader)
        - \[x\]
          [StarFlares](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/StarFlares.gdshader)
    - \[x\] Create modular planet config system using
      [ron](https://github.com/ron-rs/ron)
    - \[ \] Create planet editor app
  - \[ \] Map generation
  - \[x\] Multiplayer using [matchbox](https://docs.rs/bevy_matchbox)
- \[ \] UI
  - \[ \] Main menu
  - \[ \] Server browser menu
  - \[ \] Pause menu
  - \[ \] Settings menu
- \[ \] Tutorials:
  - \[ \] Enter orbit
  - \[ \] Use shotgun to jump and move
