use crate::core::{gravity::Mass, physics::Position};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use materials::PlanetMaterialLayerInit;
use rand::seq::SliceRandom;
use std::f64::consts::PI;

mod config;
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
        app.add_plugins(RonAssetPlugin::<config::PlanetsConfig>::new(&[]))
            .add_plugins(materials::PlanetMaterialsPlugin)
            .add_event::<SpawnPlanetEvent>()
            .add_systems(Startup, load_planets_config)
            .add_systems(Update, handle_spawn_planet_event);
    }
}

fn load_planets_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planets_config = PlanetsConfigHandle(asset_server.load("planet_kinds.ron"));
    commands.insert_resource(planets_config);
}

fn handle_spawn_planet_event(
    mut events: EventReader<SpawnPlanetEvent>,
    planet_config: Res<PlanetsConfigHandle>,
    mut commands: Commands,
    planet_configs: Res<Assets<config::PlanetsConfig>>,
) {
    for _ in events.read() {
        if let Some(config) = planet_configs.get(planet_config.0.id()) {
            if let Some(kind) = config.0.choose(&mut rand::thread_rng()) {
                // Spawn the planet's material layers
                let layers: Vec<_> = kind
                    .layers
                    .iter()
                    .enumerate()
                    .map(|(i, layer)| {
                        let scale = layer.scale.unwrap_or(1.0);
                        let z_index = i as f32;
                        let layer_commands = match layer.material.clone() {
                            config::PlanetLayerMaterialConfig::Under(config) => commands.spawn(
                                PlanetMaterialLayerInit::<materials::UnderMaterial> {
                                    config,
                                    scale,
                                    z_index,
                                },
                            ),
                            config::PlanetLayerMaterialConfig::Landmasses(config) => commands
                                .spawn(PlanetMaterialLayerInit::<materials::LandmassesMaterial> {
                                    config,
                                    scale,
                                    z_index,
                                }),
                            config::PlanetLayerMaterialConfig::Clouds(config) => commands.spawn(
                                PlanetMaterialLayerInit::<materials::CloudsMaterial> {
                                    config,
                                    scale,
                                    z_index,
                                },
                            ),
                            config::PlanetLayerMaterialConfig::Craters(config) => {
                                commands.spawn(PlanetMaterialLayerInit::<
                                    materials::CratersMaterial,
                                > {
                                    config,
                                    scale,
                                    z_index,
                                })
                            }
                            config::PlanetLayerMaterialConfig::DryTerrain(config) => commands
                                .spawn(PlanetMaterialLayerInit::<materials::DryTerrainMaterial> {
                                    config,
                                    scale,
                                    z_index,
                                }),
                        };

                        layer_commands.id()
                    })
                    .collect();

                // Spawn the planet
                let mut planet_commands = commands.spawn(PlanetBundle {
                    ..Default::default()
                });

                // Add layers as childs
                layers.into_iter().for_each(|layer| {
                    planet_commands.add_child(layer);
                });
            } else {
                warn!("Received SpawnPlanetEvent on an empty PlanetKindConfig set");
            }
        } else {
            warn!("Received SpawnPlanetEvent with no PlanetsConfig asset available");
        }
    }
}

fn radius_to_mass(radius: u32) -> u32 {
    (PI * radius.pow(2) as f64) as u32
}

#[derive(Resource)]
struct PlanetsConfigHandle(Handle<config::PlanetsConfig>);

#[derive(Event)]
pub struct SpawnPlanetEvent;
