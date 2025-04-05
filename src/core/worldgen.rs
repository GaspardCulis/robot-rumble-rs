use bevy::prelude::*;
use rand::Rng;
use rand_xoshiro::{rand_core::SeedableRng as _, Xoshiro256PlusPlus};
use serde::{Deserialize, Serialize};

use crate::entities::planet::{PlanetType, Radius, SpawnPlanetEvent};

use super::physics::Position;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateWorldEvent>()
            .add_systems(Update, handle_genworld_event);
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WorldgenConfig {
    central_star_radius: u32,

    min_planets: u32,
    max_planets: u32,

    min_planet_radius: u32,
    max_planet_radius: u32,

    max_planet_surface_distance: u32,
    min_planet_surface_distance: u32,
}

// TODO: Actual ron config file
const WORLDGEN_CONFIG: WorldgenConfig = WorldgenConfig {
    central_star_radius: 100,

    min_planets: 8,
    max_planets: 10,

    min_planet_radius: 80,
    max_planet_radius: 400,

    max_planet_surface_distance: 2000,
    min_planet_surface_distance: 100,
};

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationSeed(pub u64);

#[derive(Event)]
pub struct GenerateWorldEvent {
    pub seed: u64,
}

fn handle_genworld_event(
    mut events: EventReader<GenerateWorldEvent>,
    mut planet_spawn_events: EventWriter<SpawnPlanetEvent>,
) {
    for GenerateWorldEvent { seed } in events.read() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(*seed);
        let mut planets = Vec::new();
        planets.push(SpawnPlanetEvent {
            position: Position(Vec2::ZERO),
            radius: Radius(WORLDGEN_CONFIG.central_star_radius),
            r#type: PlanetType::Star,
            seed: rng.random(),
        });

        let num_planets: u32 =
            rng.random_range(WORLDGEN_CONFIG.min_planets..WORLDGEN_CONFIG.max_planets);
        for i in 0..num_planets {
            info!("Generating planet [{}/{}]", i + 1, num_planets);

            loop {
                let random_direction = Vec2::from_angle(rng.random_range(1..360) as f32);
                let radius = Radius(rng.random_range(
                    WORLDGEN_CONFIG.min_planet_radius..WORLDGEN_CONFIG.max_planet_radius,
                ));
                let distance = rng.random_range(
                    WORLDGEN_CONFIG.min_planet_surface_distance
                        ..WORLDGEN_CONFIG.max_planet_surface_distance,
                );
                let position = Position(random_direction * distance as f32);

                let mut collision = false;
                planets.iter().for_each(|planet| {
                    collision |= planet.position.0.distance(position.0)
                        < (planet.radius.0 + radius.0) as f32 * 1.5
                });

                if !collision {
                    planets.push(SpawnPlanetEvent {
                        position,
                        radius,
                        r#type: PlanetType::Planet,
                        seed: rng.random(),
                    });
                    break;
                }
            }
        }

        planets.into_iter().for_each(|spawn_event| {
            planet_spawn_events.send(spawn_event);
        });
    }
}
