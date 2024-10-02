use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{PlanetMaterial, PlanetMaterialLayerInit};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct CratersMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border: f32,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(1)]
    pub color2: LinearRgba,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct CratersMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    light_border: f32,
    palette: PaletteConfig2,
}

impl Material2d for CratersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/craters.wgsl".into()
    }
}

impl PlanetMaterial for CratersMaterial {
    type Config = CratersMaterialConfig;

    fn from_layer_init(
        layer_init: &PlanetMaterialLayerInit<Self>,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        let config = &layer_init.config;
        Self {
            common: super::CommonMaterial {
                size: config.size * layer_init.scale,
                octaves: config.octaves,
                ..Default::default()
            },
            light_border: config.light_border,
            color1: Srgba::hex(&config.palette[0]).unwrap().into(),
            color2: Srgba::hex(&config.palette[1]).unwrap().into(),
        }
    }
}
