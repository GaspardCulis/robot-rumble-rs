use bevy::prelude::*;

use super::materials::*;

pub mod types {
    pub type ColorConfig = String;
    pub type PaletteConfig2 = [ColorConfig; 2];
    pub type PaletteConfig3 = [ColorConfig; 3];
    pub type PaletteConfig4 = [ColorConfig; 4];

    #[derive(serde::Deserialize)]
    pub struct ColorGradientConfig {
        pub offsets: Vec<f32>,
        pub colors: Vec<ColorConfig>,
    }
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct PlanetKindConfig {
    r#type: PlanetTypeConfig,
    layers: Vec<PlanetLayerConfig>,
}

#[derive(serde::Deserialize)]
pub enum PlanetTypeConfig {
    Planet,
    Star,
}

#[derive(serde::Deserialize)]
pub struct PlanetLayerConfig {
    /// Defaults to 1
    scale: Option<f32>,
    material: PlanetLayerMaterialConfig,
}

#[derive(serde::Deserialize)]
pub enum PlanetLayerMaterialConfig {
    Under(<UnderMaterial as PlanetMaterial>::Config),
    Landmasses(<LandmassesMaterial as PlanetMaterial>::Config),
    Clouds(<CloudsMaterial as PlanetMaterial>::Config),
    Craters(<CratersMaterial as PlanetMaterial>::Config),
    DryTerrain(<DryTerrainMaterial as PlanetMaterial>::Config),
}
