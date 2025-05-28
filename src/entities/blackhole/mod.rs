use crate::core::{
    collision::CollisionShape, gravity::Mass, physics::Position, worldgen::GenerationSeed,
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::{SeedableRng, seq::IndexedRandom as _};
use rand_xoshiro::Xoshiro256PlusPlus;

// TODO: move to config
pub const BLACKHOLE_MASS: Mass = Mass(1000);

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
#[require(Visibility)]
pub struct BlackHole;

#[derive(Event)]
pub struct SpawnBlackHoleEvent {
    pub position: Position,
    pub radius: Radius,
    pub seed: u64,
    // TODO: add DecayTimer
}

#[derive(Bundle)]
struct BlackHoleBundle {
    marker: BlackHole,
    position: Position,
    radius: Radius,
    mass: Mass,
}

impl BlackHoleBundle {
    fn new(position: Position, radius: Radius, mass: Mass) -> Self {
        Self {
            marker: BlackHole,
            position,
            mass,
            collision_shape: CollisionShape::Circle(radius.0 as f32),
        }
    }
}

pub struct BlackHolePlugin;
impl Plugin for BlackHolePlugin {
    fn build(&self, app: &mut App) {
        app.register_type(BlackHole)
            .register_required_components_with::<BlackHole, Static>((|| Static))
            .register_required_components_with::<BlackHole, Name>(|| Name::new("Blackhole"))
            .add_event(SpawnBlackHoleEvent);
    }
}
