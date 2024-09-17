use crate::core::{gravity::Mass, physics::Position};
use bevy::prelude::*;
use std::f64::consts::PI;

mod kinds;
mod materials;

const DEFAULT_RADIUS: u32 = 128;

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Radius(pub u32);

#[derive(Bundle)]
struct PlanetBundle {
    marker: Planet,
    position: Position,
    radius: Radius,
    mass: Mass,
    spatial: SpatialBundle,
}

impl Default for PlanetBundle {
    fn default() -> Self {
        Self {
            marker: Planet,
            position: Position(Vec2::ZERO),
            radius: Radius(DEFAULT_RADIUS),
            mass: Mass(radius_to_mass(DEFAULT_RADIUS)),
            spatial: SpatialBundle {
                transform: Transform::from_scale(Vec3::splat((DEFAULT_RADIUS * 2) as f32)),
                ..Default::default()
            },
        }
    }
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(materials::PlanetMaterialsPlugin)
            .add_systems(Startup, spawn_planet);
    }
}

fn spawn_planet(mut commands: Commands) {
    let kind = kinds::EarthLike::new();

    commands
        .spawn(PlanetBundle {
            ..Default::default()
        })
        .insert(kind);
}

fn radius_to_mass(radius: u32) -> u32 {
    (PI * radius.pow(2) as f64) as u32
}
