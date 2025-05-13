use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::Rng;
use rand_xoshiro::{rand_core::SeedableRng as _, Xoshiro256PlusPlus};
use serde::{Deserialize, Serialize};

use crate::entities::planet::{PlanetType, Radius, SpawnPlanetEvent};
use crate::entities::satellite::SpawnSatelliteEvent;


use super::physics::Position;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<WorldgenConfig>::new(&[]))
            .add_event::<GenerateWorldEvent>()
            .add_systems(Startup, load_worldgen_config)
            .add_systems(Update, handle_genworld_event);
    }
}

fn load_worldgen_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config: Handle<WorldgenConfig> = asset_server.load("config/worldgen.ron");
    commands.insert_resource(WorldgenConfigHandle(config));
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

    // Satellite generation
    min_satellites: u32,
    max_satellites: u32,

    satellite_min_distance: f32,
    satellite_max_distance: f32,

    satellite_planet_min_distance: f32,
    satellite_satellite_min_distance: f32,
}

#[derive(Resource)]
struct WorldgenConfigHandle(pub Handle<WorldgenConfig>);

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationSeed(pub u64);

#[derive(Event)]
pub struct GenerateWorldEvent {
    pub seed: u64,
}

fn handle_genworld_event(
    mut events: EventReader<GenerateWorldEvent>,
    mut planet_spawn_events: EventWriter<SpawnPlanetEvent>,
    mut satellite_spawn_events: EventWriter<SpawnSatelliteEvent>,
    config_handle: Res<WorldgenConfigHandle>,
    configs: Res<Assets<WorldgenConfig>>,
) {
    let Some(worldgen_config) = configs.get(&config_handle.0) else {
        warn!("Worldgen config not loaded yet");
        return;
    };

    for GenerateWorldEvent { seed } in events.read() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(*seed);
        let mut planets = Vec::new();
        planets.push(SpawnPlanetEvent {
            position: Position(Vec2::ZERO),
            radius: Radius(worldgen_config.central_star_radius),
            r#type: PlanetType::Star,
            seed: rng.random(),
        });

        let num_planets: u32 =
            rng.random_range(worldgen_config.min_planets..worldgen_config.max_planets);
        for i in 0..num_planets {
            info!("Generating planet [{}/{}]", i + 1, num_planets);

            loop {
                let random_direction = Vec2::from_angle(rng.random_range(1..360) as f32);
                let radius = Radius(rng.random_range(
                    worldgen_config.min_planet_radius..worldgen_config.max_planet_radius,
                ));
                let distance = rng.random_range(
                    worldgen_config.min_planet_surface_distance
                        ..worldgen_config.max_planet_surface_distance,
                );
                let position = Position(random_direction * distance as f32);

                let mut collision = false;
                planets.iter().for_each(|planet| {
                    collision |= planet.position.0.distance(position.0)
                        < (planet.radius.0 + radius.0) as f32 * 1.5
                });

                if !collision {
                    planets.push(SpawnPlanetEvent {
                        position: position.clone(),
                        radius,
                        r#type: PlanetType::Planet,
                        seed: rng.random(),
                    });


                    break;
                }
            }
        }

       for spawn_event in &planets {
            planet_spawn_events.send(spawn_event.clone());
        }

        // Générer un certain nombre de satellites 
        let mut satellite_positions: Vec<Position> = Vec::new();
        let num_satellites = rng.random_range(
            worldgen_config.min_satellites..worldgen_config.max_satellites,
        );

        for _ in 0..num_satellites {
            let mut attempts = 0;
            loop {
                attempts += 1;
                if attempts > 10 {
                    warn!("Could not place satellite after 10 attempts, skipping...");
                    break;
                }

                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let distance = rng.random_range(
                    worldgen_config.satellite_min_distance..worldgen_config.satellite_max_distance,
                );
                let position = Position(Vec2::from_angle(angle) * distance);

                let safe_distance_planet = worldgen_config.satellite_planet_min_distance;
                let safe_distance_satellite = worldgen_config.satellite_satellite_min_distance;

                let far_from_planets = planets.iter().all(|planet| {
                    position.0.distance(planet.position.0) > (planet.radius.0 as f32 + safe_distance_planet)
                });

                let far_from_satellites = satellite_positions.iter().all(|existing| {
                    position.0.distance(existing.0) > safe_distance_satellite
                });

                if far_from_planets && far_from_satellites {
                    satellite_spawn_events.send(SpawnSatelliteEvent {
                        position: position.clone(),
                        scale: rng.random_range(0.5..0.9),
                    });
                    satellite_positions.push(position);
                    break;
                }
            }
        }


    }
}
