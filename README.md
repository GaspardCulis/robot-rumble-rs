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
    - [x] Create modular planet config system using [ron](https://github.com/ron-rs/ron)
  - [ ] Map generation
- [x] Multiplayer using [lightyear](https://docs.rs/lightyear)
- [ ] UI
  - [ ] Main menu
  - [ ] Server browser menu
  - [ ] Pause menu
  - [ ] Settings menu
- [ ] Tutorials:
  - [ ] Enter orbit
  - [ ] Use shotgun to jump and move
