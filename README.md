# robot-rumble-rs

The Rust rewrite of [Robot Rumble](https://github.com/GaspardCulis/robot-rumble)

## TODOS

- [ ] Player:
  - [x] Sprites and animations
  - [ ] Keep angular velocity on lost
- [ ] Weapons:
  - [ ] Shotgun
  - [ ] Uzi
  - [ ] Black hole launcher
  - [ ] Rocket launcher
  - [ ] Lazer gun
- [ ] Planets and environment:
  - [x] Implement [pixel planet](https://deep-fold.itch.io/pixel-planet-generator) shader
  - [ ] Add background shader using [pixel space background](https://deep-fold.itch.io/space-background-generator) shader
  - [ ] Add multiple kinds of planets
    - [ ] Implement all shader types
      - [ ] [Asteroids](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Asteroids/Asteroids.gdshader)
      - [ ] BlackHole
        - [ ] [BlackHole](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/BlackHole/BlackHole.gdshader)
        - [ ] [BlackHoleRing](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/BlackHole/BlackHoleRing.gdshader)
      - [ ] [DryTerrain](https://github.com/Deep-Fold/PixelPlanets/tree/main/Planets/DryTerran) ?
      - [ ] [Galaxy](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Galaxy/Galaxy.gdshader)
      - [ ] [GasPlanet](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanet/GasPlanet.gdshader)
      - [ ] SaturnLike
        - [ ] [GasLayers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanetLayers/GasLayers.gdshader)
        - [ ] [Ring](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/GasPlanetLayers/Ring.gdshader)
      - [ ] [IceWorld](https://github.com/Deep-Fold/PixelPlanets/tree/main/Planets/IceWorld) ?
      - [ ] [Clouds](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/Clouds.gdshader)
      - [ ] [PlanetLandmass](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/PlanetLandmass.gdshader)
      - [ ] [PlanetUnder](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LandMasses/PlanetUnder.gdshader)
      - [ ] [Rivers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/LavaWorld/Rivers.gdshader)
      - [ ] [Craters](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/NoAtmosphere/Craters.gdshader)
      - [ ] [NoAtmosphere](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/NoAtmosphere/NoAtmosphere.gdshader)
      - [ ] [LandRivers](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Rivers/LandRivers.gdshader)
      - [ ] Star
        - [ ] [Star](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/Star.gdshader)
        - [ ] [StarBlobs](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/StarBlobs.gdshader)
        - [ ] [StarFlares](https://github.com/Deep-Fold/PixelPlanets/blob/main/Planets/Star/StarFlares.gdshader)
    - [x] Create modular planet config system using [ron](https://github.com/ron-rs/ron)
    - [ ] Create planet editor app
  - [ ] Map generation
  - [x] Multiplayer using [matchbox](https://docs.rs/bevy_matchbox)
- [ ] UI
  - [ ] Main menu
  - [ ] Server browser menu
  - [ ] Pause menu
  - [ ] Settings menu
- [ ] Tutorials:
  - [ ] Enter orbit
  - [ ] Use shotgun to jump and move
