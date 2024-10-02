use bevy::prelude::*;

use super::materials::*;

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
        app.add_systems(Startup, load_planets_config);
    }
}

fn load_planets_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventWriter<super::SpawnPlanetEvent>,
) {
    let planets_config = PlanetsConfigHandle(asset_server.load("planet_kinds.ron"));
    commands.insert_resource(planets_config);

    events.send(super::SpawnPlanetEvent);
}

#[derive(Resource)]
pub struct PlanetsConfigHandle(pub Handle<PlanetsConfig>);
