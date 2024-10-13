use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use rand::seq::SliceRandom;

use super::{
    materials::{self, *},
    Planet,
};

pub mod types {
    pub type ColorConfig = String;
    pub type PaletteConfig2 = [ColorConfig; 2];
    pub type PaletteConfig3 = [ColorConfig; 3];
    pub type PaletteConfig4 = [ColorConfig; 4];

    #[derive(serde::Deserialize, Clone)]
    pub struct ColorGradientConfig {
        pub offsets: Vec<f32>,
        pub colors: Vec<ColorConfig>,
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct PlanetsConfig(pub Vec<PlanetKindConfig>);

#[derive(serde::Deserialize)]
pub struct PlanetKindConfig {
    pub r#type: PlanetTypeConfig,
    pub layers: Vec<PlanetLayerConfig>,
}

#[derive(serde::Deserialize)]
pub enum PlanetTypeConfig {
    Planet,
    Star,
}

#[derive(serde::Deserialize)]
pub struct PlanetLayerConfig {
    /// Defaults to 1
    pub scale: Option<f32>,
    pub material: PlanetLayerMaterialConfig,
}

#[derive(serde::Deserialize, Clone)]
pub enum PlanetLayerMaterialConfig {
    Under(<UnderMaterial as PlanetMaterial>::Config),
    Landmasses(<LandmassesMaterial as PlanetMaterial>::Config),
    Lakes(<LakesMaterial as PlanetMaterial>::Config),
    Clouds(<CloudsMaterial as PlanetMaterial>::Config),
    Craters(<CratersMaterial as PlanetMaterial>::Config),
    DryTerrain(<DryTerrainMaterial as PlanetMaterial>::Config),
    GasLayers(<GasLayersMaterial as PlanetMaterial>::Config),
    Ring(<RingMaterial as PlanetMaterial>::Config),
}

pub struct PlanetsConfigPlugin;

impl Plugin for PlanetsConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<PlanetsConfig>::new(&[]))
            .add_systems(Startup, load_planets_config)
            .add_systems(Update, spawn_config_layers);
    }
}

fn load_planets_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planets_config = PlanetsConfigHandle(asset_server.load("planet_kinds.ron"));
    commands.insert_resource(planets_config);
}

fn spawn_config_layers(
    mut commands: Commands,
    planet_config: Res<PlanetsConfigHandle>,
    planet_configs: Res<Assets<PlanetsConfig>>,
    query: Query<Entity, Added<Planet>>,
) {
    for planet_entity in query.iter() {
        let mut planet = commands.entity(planet_entity);
        // Get config
        if let Some(config) = planet_configs.get(planet_config.0.id()) {
            if let Some(kind) = config.0.choose(&mut rand::thread_rng()) {
                // Spawn the planet's material layers
                for (i, layer) in kind.layers.iter().enumerate() {
                    let scale = layer.scale.unwrap_or(1.0);
                    let z_index = i as f32 * 0.001;
                    match layer.material.clone() {
                        PlanetLayerMaterialConfig::Under(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::UnderMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Landmasses(config) => planet.insert(
                            PlanetMaterialLayerInit::<materials::LandmassesMaterial> {
                                config,
                                scale,
                                z_index,
                            },
                        ),
                        PlanetLayerMaterialConfig::Clouds(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::CloudsMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Craters(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::CratersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::DryTerrain(config) => planet.insert(
                            PlanetMaterialLayerInit::<materials::DryTerrainMaterial> {
                                config,
                                scale,
                                z_index,
                            },
                        ),
                        PlanetLayerMaterialConfig::Lakes(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::LakesMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::GasLayers(config) => {
                            planet.insert(PlanetMaterialLayerInit::<materials::GasLayersMaterial> {
                                config,
                                scale,
                                z_index,
                            })
                        }
                        PlanetLayerMaterialConfig::Ring(config) => {
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

#[derive(Resource)]
struct PlanetsConfigHandle(Handle<PlanetsConfig>);
