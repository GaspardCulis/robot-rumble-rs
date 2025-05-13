use crate::core::{
    gravity::Mass,
    physics::Position,
    worldgen::{self, GenerationSeed},
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::{seq::IndexedRandom as _, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

mod config;
mod materials;

use config::*;
use materials::*;

#[derive(Component, Debug, Reflect, Clone, PartialEq)]
#[require(Visibility)]
pub struct Planet;

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq)]
pub struct Radius(pub u32);

#[derive(Component, Debug, Reflect, Copy, Clone, PartialEq, serde::Deserialize)]
pub enum PlanetType {
    Planet,
    Star,
}

#[derive(Event)]
pub struct SpawnPlanetEvent {
    pub position: Position,
    pub radius: Radius,
    pub r#type: PlanetType,
    pub seed: u64,
}

#[derive(Bundle)]
struct PlanetBundle {
    name: Name,
    marker: Planet,
    position: Position,
    radius: Radius,
    r#type: PlanetType,
    mass: Mass,
}

impl PlanetBundle {
    fn new(position: Position, radius: Radius, r#type: PlanetType) -> Self {
        Self {
            position,
            radius,
            r#type,
            name: Name::new("Planet"),
            marker: Planet,
            mass: Mass(radius_to_mass(radius)),
        }
    }
}

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PlanetsConfig>::new(&[]))
            .add_plugins(materials::PlanetMaterialsPlugin)
            .add_event::<SpawnPlanetEvent>()
            .add_systems(Startup, load_planets_config)
            .add_systems(Update, (handle_spawn_planet_event, spawn_config_layers));
    }
}

fn load_planets_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planets_config = PlanetsConfigHandle(asset_server.load("config/planet_kinds.ron"));
    commands.insert_resource(planets_config);
}

fn handle_spawn_planet_event(mut events: EventReader<SpawnPlanetEvent>, mut commands: Commands) {
    for event in events.read() {
        commands.spawn((
            PlanetBundle::new(event.position.clone(), event.radius, event.r#type),
            GenerationSeed(event.seed),
        ));
    }
}

fn spawn_config_layers(
    mut commands: Commands,
    planet_config: Res<PlanetsConfigHandle>,
    planet_configs: Res<Assets<PlanetsConfig>>,
    query: Query<(Entity, &PlanetType, Option<&worldgen::GenerationSeed>), Added<Planet>>,
) {
    for (planet_entity, planet_type, generation_seed) in query.iter() {
        let mut planet = commands.entity(planet_entity);
        let mut rng = match generation_seed {
            Some(seed) => Xoshiro256PlusPlus::seed_from_u64(seed.0),
            None => Xoshiro256PlusPlus::from_os_rng(),
        };

        // Get config
        if let Some(config) = planet_configs.get(planet_config.0.id()) {
            if let Some(kind) = config
                .0
                .iter()
                .filter(|c| c.r#type == *planet_type)
                .collect::<Vec<_>>()
                .choose(&mut rng)
            {
                // Spawn the planet's material layers
                for (i, layer) in kind.layers.iter().enumerate() {
                    let scale = layer.scale.unwrap_or(1.0);
                    let z_index = i as f32 * 0.001;
                    match layer.material.clone() {
                        PlanetLayerMaterialConfig::Under(config) => {
                            planet.insert(PlanetMaterialLayerInit::<UnderMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Landmasses(config) => {
                            planet.insert(PlanetMaterialLayerInit::<LandmassesMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Clouds(config) => {
                            planet.insert(PlanetMaterialLayerInit::<CloudsMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Craters(config) => {
                            planet.insert(PlanetMaterialLayerInit::<CratersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::DryTerrain(config) => {
                            planet.insert(PlanetMaterialLayerInit::<DryTerrainMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Lakes(config) => {
                            planet.insert(PlanetMaterialLayerInit::<LakesMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::GasLayers(config) => {
                            planet.insert(PlanetMaterialLayerInit::<GasLayersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Ring(config) => {
                            planet.insert(PlanetMaterialLayerInit::<RingMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Star(config) => {
                            planet.insert(PlanetMaterialLayerInit::<StarMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::StarFlares(config) => {
                            planet.insert(PlanetMaterialLayerInit::<StarFlaresMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::StarBlobs(config) => {
                            planet.insert(PlanetMaterialLayerInit::<StarBlobsMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::BlackHole(config) => {
                            planet.insert(PlanetMaterialLayerInit::<BlackHoleMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::BlackHoleRing(config) => {
                            planet.insert(PlanetMaterialLayerInit::<BlackHoleRingMaterial> {
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

#[derive(Resource)]
struct PlanetsConfigHandle(Handle<PlanetsConfig>);

fn radius_to_mass(radius: Radius) -> u32 {
    (std::f64::consts::PI * radius.0.pow(2) as f64) as u32
}
