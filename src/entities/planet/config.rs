use bevy::{prelude::*, sprite::Material2d};

use super::materials::*;

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

type ColorConfig = String;

type PaletteConfig2 = [ColorConfig; 2];
type PaletteConfig3 = [ColorConfig; 3];
type PaletteConfig4 = [ColorConfig; 4];

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

#[derive(serde::Deserialize)]
struct ColorGradientConfig {
    offsets: Vec<f32>,
    colors: Vec<ColorConfig>,
}

/*
#[derive(Bundle)]
pub struct EarthLike {
    ocean: PlanetMaterialLayerInit<UnderMaterial>,
    land: PlanetMaterialLayerInit<LandmassesMaterial>,
    clouds: PlanetMaterialLayerInit<CloudsMaterial>,
}

impl EarthLike {
    pub fn new() -> Self {
        Self {
            ocean: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: -2.,
            },
            land: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: -1.,
            },
            clouds: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: 0.,
            },
        }
    }
}

#[derive(Bundle)]
pub struct MoonLike {
    under: PlanetMaterialLayerInit<UnderMaterial>,
    craters: PlanetMaterialLayerInit<CratersMaterial>,
}

impl MoonLike {
    pub fn new() -> Self {
        Self {
            under: PlanetMaterialLayerInit {
                material: UnderMaterial {
                    common: CommonMaterial {
                        size: 8.,
                        light_origin: Vec2::new(0.25, 0.25),
                        ..Default::default()
                    },
                    color1: Srgba::hex("#a3a7c2").unwrap().into(),
                    color2: Srgba::hex("#4c6885").unwrap().into(),
                    color3: Srgba::hex("#3a3f5e").unwrap().into(),
                    light_border_1: 0.615,
                    light_border_2: 0.729,
                    ..Default::default()
                },
                scale: 1.,
                z_index: -1.,
            },

            craters: PlanetMaterialLayerInit {
                material: Default::default(),
                scale: 1.,
                z_index: 0.,
            },
        }
    }
}
*/
