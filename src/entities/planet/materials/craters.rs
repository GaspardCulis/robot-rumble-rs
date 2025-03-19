use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{CommonMaterial, PlanetMaterial};

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
    octaves: i32,
    // Material specific
    light_border: f32,
    palette: PaletteConfig2,
}

impl Material2d for CratersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/craters.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for CratersMaterial {
    type Config = CratersMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.octaves = config.octaves;

        Self {
            common,
            light_border: config.light_border,
            color1: Srgba::hex(&config.palette[0]).unwrap().into(),
            color2: Srgba::hex(&config.palette[1]).unwrap().into(),
        }
    }
}
