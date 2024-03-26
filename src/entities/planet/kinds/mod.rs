use bevy::prelude::*;

pub trait PlanetKind: Bundle {
    fn generate() -> Self;
}
