use bevy::{asset::Asset, reflect::TypePath};

use super::{materials::*, PlanetType};

pub mod types {
    pub type ColorConfig = String;
    pub type PaletteConfig2 = [ColorConfig; 2];
    pub type PaletteConfig3 = [ColorConfig; 3];
    pub type PaletteConfig4 = [ColorConfig; 4];
    pub type PaletteConfig5 = [ColorConfig; 5];

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
    pub r#type: PlanetType,
    pub layers: Vec<PlanetLayerConfig>,
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
    Star(<StarMaterial as PlanetMaterial>::Config),
    StarBlobs(<StarBlobsMaterial as PlanetMaterial>::Config),
    StarFlares(<StarFlaresMaterial as PlanetMaterial>::Config),
    Ring(<RingMaterial as PlanetMaterial>::Config),
    BlackHole(<BlackHoleMaterial as PlanetMaterial>::Config),
    BlackHoleRing(<BlackHoleRingMaterial as PlanetMaterial>::Config),
}
