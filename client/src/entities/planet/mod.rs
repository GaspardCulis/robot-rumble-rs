use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

mod config;
mod materials;

use config::*;
use materials::*;
use rand::{seq::SliceRandom as _, SeedableRng as _};
use robot_rumble_common::{core::worldgen, entities::planet::*};

pub struct ClientPlanetPlugin;
impl Plugin for ClientPlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PlanetsConfig>::new(&[]))
            .add_plugins(materials::PlanetMaterialsPlugin)
            .add_systems(Startup, load_planets_config)
            .add_systems(Update, (spawn_config_layers, add_spacial_bundle));
    }
}

fn load_planets_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planets_config = PlanetsConfigHandle(asset_server.load("config/planet_kinds.ron"));
    commands.insert_resource(planets_config);
}

fn spawn_config_layers(
    mut commands: Commands,
    planet_config: Res<PlanetsConfigHandle>,
    planet_configs: Res<Assets<PlanetsConfig>>,
    query: Query<(Entity, Option<&worldgen::GenerationSeed>), Added<Planet>>,
) {
    for (planet_entity, generation_seed) in query.iter() {
        let mut planet = commands.entity(planet_entity);
        let mut rng = match generation_seed {
            Some(seed) => rand::rngs::StdRng::seed_from_u64(seed.0),
            None => rand::rngs::StdRng::from_entropy(),
        };

        // Get config
        if let Some(config) = planet_configs.get(planet_config.0.id()) {
            if let Some(kind) = config.0.choose(&mut rng) {
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

fn add_spacial_bundle(mut commands: Commands, query: Query<(Entity, &Radius), Added<Planet>>) {
    for (planet_entity, radius) in query.iter() {
        let mut planet_commands = commands.entity(planet_entity);
        planet_commands.insert((
            Transform::from_scale(Vec3::splat((radius * 2).0 as f32)),
            Visibility::Visible,
        ));
    }
}

#[derive(Resource)]
struct PlanetsConfigHandle(Handle<PlanetsConfig>);
