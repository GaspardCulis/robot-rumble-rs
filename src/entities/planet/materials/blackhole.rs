use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct BlackHoleMaterial {
    #[uniform(0)]
    pub common: CommonMaterial,

    #[uniform(1)]
    pub radius: f32,
    #[uniform(1)]
    pub light_width: f32,

    #[uniform(1)]
    pub color_core: LinearRgba,
    #[uniform(1)]
    pub color_inner: LinearRgba,
    #[uniform(1)]
    pub color_outer: LinearRgba,

    #[uniform(1)]
    _wasm_padding: Vec2,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct BlackHoleMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Specific to black hole
    radius: f32,
    light_width: f32,
    palette: PaletteConfig3,
}

impl Material2d for BlackHoleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/black_hole.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for BlackHoleMaterial {
    type Config = BlackHoleMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.size = config.size;
        common.octaves = config.octaves;

        Self {
            common,
            radius: config.radius,
            light_width: config.light_width,
            color_core: Srgba::hex(&config.palette[0]).unwrap().into(),
            color_inner: Srgba::hex(&config.palette[1]).unwrap().into(),
            color_outer: Srgba::hex(&config.palette[2]).unwrap().into(),
            _wasm_padding: Vec2::ZERO,
        }
    }
}
