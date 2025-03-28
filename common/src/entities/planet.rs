use crate::core::{gravity::Mass, physics::Position, worldgen::GenerationSeed};
use bevy::prelude::*;
use lightyear::prelude::server;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Component, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize)]
pub struct Planet;

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct Radius(pub u32);

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub position: Position,
    pub radius: Radius,
    pub seed: u64,
}

#[derive(Bundle)]
struct PlanetBundle {
    name: Name,
    marker: Planet,
    position: Position,
    radius: Radius,
    mass: Mass,
}

impl PlanetBundle {
    fn new(position: Position, radius: Radius) -> Self {
        Self {
            position,
            radius,
            name: Name::new("Planet"),
            marker: Planet,
            mass: Mass(radius_to_mass(radius)),
        }
    }
}

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlanetEvent>()
            .add_systems(Update, handle_spawn_planet_event);
    }
}

fn handle_spawn_planet_event(mut events: EventReader<SpawnPlanetEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.spawn((
            PlanetBundle::new(event.position.clone(), event.radius),
            GenerationSeed(event.seed),
            server::Replicate::default(),
        ));
    }
}

fn radius_to_mass(radius: Radius) -> u32 {
    (PI * (radius * radius).0 as f64) as u32
}

impl std::ops::Add for Radius {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Radius(self.0 + rhs.0)
    }
}

impl std::ops::Mul for Radius {
    type Output = Radius;

    fn mul(self, rhs: Self) -> Self::Output {
        Radius(self.0 * rhs.0)
    }
}

impl std::ops::Mul<u32> for &Radius {
    type Output = Radius;

    fn mul(self, rhs: u32) -> Self::Output {
        Radius(self.0 * rhs)
    }
}
