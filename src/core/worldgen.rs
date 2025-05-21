use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::Rng;
use rand_xoshiro::{Xoshiro256PlusPlus, rand_core::SeedableRng as _};
use serde::{Deserialize, Serialize};

use crate::entities::planet::*;

use super::physics::Position;

pub struct WorldgenPlugin;
impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<WorldgenConfig>::new(&[]))
            .add_event::<GenerateWorldEvent>()
            .add_systems(Startup, load_worldgen_config)
            .add_systems(
                Update,
                (
                    handle_genworld_event,
                    #[cfg(debug_assertions)]
                    handle_config_reload.run_if(resource_exists::<crate::network::SessionSeed>),
                ),
            );
    }
}

fn load_worldgen_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config: Handle<WorldgenConfig> = asset_server.load("config/worldgen.ron");
    commands.insert_resource(WorldgenConfigHandle(config));
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
}

#[derive(Resource)]
pub struct WorldgenConfigHandle(pub Handle<WorldgenConfig>);

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerationSeed(pub u64);

#[derive(Event)]
pub struct GenerateWorldEvent {
    pub seed: u64,
}

fn handle_genworld_event(
    mut events: EventReader<GenerateWorldEvent>,
    mut planet_spawn_events: EventWriter<SpawnPlanetEvent>,
    config_handle: Res<WorldgenConfigHandle>,
    configs: Res<Assets<WorldgenConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
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

        planets.into_iter().for_each(|spawn_event| {
            planet_spawn_events.send(spawn_event);
        });
    }
}

/// Re-generates world on config changes. Will cause desyncs
#[cfg(debug_assertions)]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<WorldgenConfig>>,
    mut worldgen_events: EventWriter<GenerateWorldEvent>,
    planets: Query<Entity, With<Planet>>,
    seed: Res<crate::network::SessionSeed>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id: _ } => {
                for planet in planets.iter() {
                    commands.entity(planet).despawn_recursive();
                }

                worldgen_events.send(GenerateWorldEvent { seed: seed.0 });
            }
            _ => {}
        };
    }
}
