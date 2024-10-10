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

#[derive(Component, Reflect, Clone)]
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
            .add_plugins(config::PlanetsConfigPlugin)
            .add_event::<SpawnPlanetEvent>()
            .add_systems(Update, handle_spawn_planet_event);
    }
}

fn handle_spawn_planet_event(
    mut events: EventReader<SpawnPlanetEvent>,
    planet_config: Res<config::PlanetsConfigHandle>,
    mut commands: Commands,
    planet_configs: Res<Assets<config::PlanetsConfig>>,
) {
    for event in events.read() {
        if let Some(config) = planet_configs.get(planet_config.0.id()) {
            if let Some(kind) = config.0.choose(&mut rand::thread_rng()) {
                // Spawn the planet
                let mut planet = commands.spawn((
                    Name::new("Planet"),
                    PlanetBundle {
                        position: event.position.clone(),
                        radius: event.radius.clone(),
                        mass: Mass(radius_to_mass(event.radius.0)),
                        spatial: SpatialBundle {
                            transform: Transform::from_scale(Vec3::splat(
                                (event.radius.0 * 2) as f32,
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ));

                // Spawn the planet's material layers
                for (i, layer) in kind.layers.iter().enumerate() {
                    let scale = layer.scale.unwrap_or(1.0);
                    let z_index = i as f32 * 0.001;
                    match layer.material.clone() {
                        config::PlanetLayerMaterialConfig::Under(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::UnderMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        config::PlanetLayerMaterialConfig::Landmasses(config) => planet.insert(
                            PlanetMaterialLayerInit::<materials::LandmassesMaterial> {
                                config,
                                scale,
                                z_index,
                            },
                        ),
                        config::PlanetLayerMaterialConfig::Clouds(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::CloudsMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        config::PlanetLayerMaterialConfig::Craters(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::CratersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        config::PlanetLayerMaterialConfig::DryTerrain(config) => planet.insert(
                            PlanetMaterialLayerInit::<materials::DryTerrainMaterial> {
                                config,
                                scale,
                                z_index,
                            },
                        ),
                        config::PlanetLayerMaterialConfig::Lakes(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::LakesMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        config::PlanetLayerMaterialConfig::GasLayers(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::GasLayersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        config::PlanetLayerMaterialConfig::Ring(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::RingMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                    };
                }
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

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub position: Position,
    pub radius: Radius,
}
