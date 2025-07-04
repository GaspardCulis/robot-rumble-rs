use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use rand::Rng;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng as _};
use serde::{Deserialize, Serialize};

use crate::entities::planet::{PlanetType, Radius, SpawnPlanetEvent};
use crate::entities::satellite::{SatelliteKind, SpawnSatelliteEvent};

use super::physics::Position;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GenerateWorldEvent>().add_systems(
            Update,
            (
                handle_genworld_event.run_if(resource_exists::<WorldgenAssets>),
                #[cfg(feature = "dev_tools")]
                handle_config_reload.run_if(resource_exists::<crate::network::SessionSeed>),
            ),
        );
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WorldgenConfig {
    pub central_star_radius: u32,

    pub min_planets: u32,
    pub max_planets: u32,

    pub min_planet_radius: u32,
    pub max_planet_radius: u32,

    pub min_planet_surface_distance: u32,

    pub edge_radius: u32,
    pub edge_margin: u32,

    // Satellite generation
    min_satellites: u32,
    max_satellites: u32,

    satellite_min_distance: f32,
    satellite_max_distance: f32,

    satellite_planet_min_distance: f32,
    satellite_satellite_min_distance: f32,
}

#[derive(AssetCollection, Resource)]
pub struct WorldgenAssets {
    #[asset(path = "config/config.worldgen.ron")]
    pub config: Handle<WorldgenConfig>,
}

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
    configs: Res<Assets<WorldgenConfig>>,
    assets: Res<WorldgenAssets>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Worldgen config not loaded yet");
        return;
    };

    for GenerateWorldEvent { seed } in events.read() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(*seed);
        let mut planets = Vec::new();
        planets.push(SpawnPlanetEvent {
            position: Position(Vec2::ZERO),
            radius: Radius(config.central_star_radius),
            r#type: PlanetType::Star,
            seed: rng.random(),
        });

        let num_planets: u32 = rng.random_range(config.min_planets..config.max_planets);
        for i in 0..num_planets {
            info!("Generating planet [{}/{}]", i + 1, num_planets);

            loop {
                let random_direction = Vec2::from_angle(rng.random_range(-PI..PI));
                let radius = rng.random_range(config.min_planet_radius..config.max_planet_radius);

                let min_distance_from_center = config.central_star_radius + radius;
                let max_distance_from_center = config.edge_radius - config.edge_margin - radius;

                let distance = rng.random_range(min_distance_from_center..max_distance_from_center);
                let position = random_direction * distance as f32;

                let valid = planets.iter().fold(true, |acc, planet| {
                    let distance_respected = planet.position.0.distance(position)
                        > (planet.radius.0 + radius + config.min_planet_surface_distance) as f32;

                    acc && distance_respected
                });

                if valid {
                    planets.push(SpawnPlanetEvent {
                        position: Position(position),
                        radius: Radius(radius),
                        r#type: PlanetType::Planet,
                        seed: rng.random(),
                    });

                    break;
                }
            }
        }

        // Générer un certain nombre de satellites
        let mut satellite_positions: Vec<Position> = Vec::new();
        let num_satellites = rng.random_range(config.min_satellites..config.max_satellites);

        for _ in 0..num_satellites {
            let mut attempts = 0;
            loop {
                attempts += 1;
                if attempts > 10 {
                    warn!("Could not place satellite after 10 attempts, skipping...");
                    break;
                }

                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let distance =
                    rng.random_range(config.satellite_min_distance..config.satellite_max_distance);
                let position = Position(Vec2::from_angle(angle) * distance);

                let safe_distance_planet = config.satellite_planet_min_distance;
                let safe_distance_satellite = config.satellite_satellite_min_distance;

                let far_from_planets = planets.iter().all(|planet| {
                    position.0.distance(planet.position.0)
                        > (planet.radius.0 as f32 + safe_distance_planet)
                });

                let far_from_satellites = satellite_positions
                    .iter()
                    .all(|existing| position.0.distance(existing.0) > safe_distance_satellite);

                let kind = match rng.random_range(0..3) {
                    0 => SatelliteKind::Slingshot,
                    1 => SatelliteKind::Bumper,
                    _ => SatelliteKind::Grabber,
                };

                if far_from_planets && far_from_satellites {
                    satellite_spawn_events.write(SpawnSatelliteEvent {
                        position: position.clone(),
                        // FIX: Config file or constant, will be done when we have better sprites
                        scale: 0.7,
                        kind,
                    });
                    satellite_positions.push(position);
                    break;
                }
            }
        }

        for spawn_event in planets {
            planet_spawn_events.write(spawn_event);
        }
    }
}

/// Re-generates world on config changes. Will cause desyncs
#[cfg(feature = "dev_tools")]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<WorldgenConfig>>,
    mut worldgen_events: EventWriter<GenerateWorldEvent>,
    entities: Query<
        Entity,
        Or<(
            With<crate::entities::planet::Planet>,
            With<crate::entities::satellite::Satellite>,
        )>,
    >,
    seed: Res<crate::network::SessionSeed>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id: _ } = event {
            for entity in entities.iter() {
                commands.entity(entity).despawn();
            }

            worldgen_events.write(GenerateWorldEvent { seed: seed.0 });
        };
    }
}
