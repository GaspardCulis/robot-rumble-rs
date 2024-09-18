use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::PlanetMaterial;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LandmassesMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub land_cutoff: f32,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(1)]
    pub color2: LinearRgba,
    #[uniform(1)]
    pub color3: LinearRgba,
    #[uniform(1)]
    pub color4: LinearRgba,
}

#[derive(serde::Deserialize, Component, Clone)]
pub struct LandmassesMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    light_border_1: f32,
    light_border_2: f32,
    land_cutoff: f32,
    palette: PaletteConfig4,
}

impl Material2d for LandmassesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/landmasses.wgsl".into()
    }
}

impl PlanetMaterial for LandmassesMaterial {
    type Config = LandmassesMaterialConfig;

    fn from_config(config: &Self::Config, _: &mut ResMut<Assets<Image>>) -> Self {
        Self {
            common: super::CommonMaterial {
                size: config.size,
                octaves: config.octaves,
                ..Default::default()
            },
            light_border_1: config.light_border_1,
            light_border_2: config.light_border_2,
            land_cutoff: config.land_cutoff,
            color1: Srgba::hex(&config.palette[0]).unwrap().into(),
            color2: Srgba::hex(&config.palette[1]).unwrap().into(),
            color3: Srgba::hex(&config.palette[2]).unwrap().into(),
            color4: Srgba::hex(&config.palette[3]).unwrap().into(),
        }
    }
}
