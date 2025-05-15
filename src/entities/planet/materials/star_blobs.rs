use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct StarBlobsMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub circle_amount: f32,
    #[uniform(1)]
    pub circle_size: f32,
    #[uniform(1)]
    pub color: LinearRgba,
    #[uniform(1)]
    _wasm_padding: Vec2,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct StarBlobsMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    circle_amount: f32,
    circle_size: f32,
    color: ColorConfig,
}

impl Material2d for StarBlobsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/star_blobs.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for StarBlobsMaterial {
    type Config = StarBlobsMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.size = config.size;
        common.octaves = config.octaves;

        Self {
            common,
            circle_amount: config.circle_amount,
            circle_size: config.circle_size,
            color: Srgba::hex(&config.color).unwrap().into(),
            _wasm_padding: Default::default(),
        }
    }
}
