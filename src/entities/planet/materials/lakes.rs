use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct LakesMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub lake_cutoff: f32,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(1)]
    pub color2: LinearRgba,
    #[uniform(1)]
    pub color3: LinearRgba,
    #[uniform(1)]
    _wasm_padding: f32,
}

#[derive(serde::Deserialize, Component, Clone)]
pub struct LakesMaterialConfig {
    // Common
    octaves: i32,
    // Material specific
    light_border_1: f32,
    light_border_2: f32,
    lake_cutoff: f32,
    palette: PaletteConfig3,
}

impl Material2d for LakesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/lakes.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for LakesMaterial {
    type Config = LakesMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.octaves = config.octaves;

        Self {
            common,
            light_border_1: config.light_border_1,
            light_border_2: config.light_border_2,
            lake_cutoff: config.lake_cutoff,
            color1: Srgba::hex(&config.palette[0]).unwrap().into(),
            color2: Srgba::hex(&config.palette[1]).unwrap().into(),
            color3: Srgba::hex(&config.palette[2]).unwrap().into(),
            _wasm_padding: 0.0,
        }
    }
}
