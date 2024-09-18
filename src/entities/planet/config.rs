use bevy::prelude::*;

use super::materials::*;

pub mod types {
    pub type ColorConfig = String;
    pub type PaletteConfig2 = [ColorConfig; 2];
    pub type PaletteConfig3 = [ColorConfig; 3];
    pub type PaletteConfig4 = [ColorConfig; 4];

    #[derive(serde::Deserialize)]
    pub struct ColorGradientConfig {
        offsets: Vec<f32>,
        colors: Vec<ColorConfig>,
    }
}

use types::*;

#[derive(serde::Deserialize, Asset, TypePath)]
struct PlanetKindConfig {
    r#type: PlanetTypeConfig,
    layers: Vec<PlanetLayerConfig>,
}

#[derive(serde::Deserialize)]
enum PlanetTypeConfig {
    Planet,
    Star,
}

#[derive(serde::Deserialize)]
struct PlanetLayerConfig {
    /// Defaults to 1
    scale: Option<f32>,
    material: PlanetLayerMaterialConfig,
}

#[derive(serde::Deserialize)]
enum PlanetLayerMaterialConfig {
    Under(<UnderMaterial as PlanetMaterial>::Config),
    Landmasses {
        // Common
        size: f32,
        octaves: i32,
        // Material specific
        light_border_1: f32,
        light_border_2: f32,
        land_cutoff: f32,
        palette: PaletteConfig4,
    },
    Clouds {
        // Common
        size: f32,
        octaves: i32,
        // Material specific
        stretch: f32,
        cloud_curve: f32,
        light_border_1: f32,
        light_border_2: f32,
        palette: PaletteConfig4,
    },
    Craters {
        // Common
        size: f32,
        octaves: i32,
        // Material specific
        light_border: f32,
        palette: PaletteConfig2,
    },
    DryTerrain {
        // Common
        size: f32,
        octaves: i32,
        // Material specific
        dither_size: f32,
        light_distance_1: f32,
        light_distance_2: f32,
        colors: ColorGradientConfig,
    },
}
