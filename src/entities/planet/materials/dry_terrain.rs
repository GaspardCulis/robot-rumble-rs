use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{entities::planet::config::types::*, utils};

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct DryTerrainMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub dither_size: f32,
    #[uniform(1)]
    pub light_distance_1: f32,
    #[uniform(1)]
    pub light_distance_2: f32,
    #[uniform(1)]
    _wasm_padding: f32,
    #[texture(2)]
    #[sampler(3)]
    color_texture: Option<Handle<Image>>,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct DryTerrainMaterialConfig {
    // Common
    octaves: i32,
    // Material specific
    dither_size: f32,
    light_distance_1: f32,
    light_distance_2: f32,
    colors: ColorGradientConfig,
}

impl Material2d for DryTerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/dry_terrain.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for DryTerrainMaterial {
    type Config = DryTerrainMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self {
        let gradient = utils::gradient(
            &config.colors.offsets,
            &config
                .colors
                .colors
                .iter()
                .map(|color| Srgba::hex(color).unwrap())
                .collect(),
        );

        common.octaves = config.octaves;

        Self {
            common,
            dither_size: config.dither_size,
            light_distance_1: config.light_distance_1,
            light_distance_2: config.light_distance_2,
            color_texture: Some(images.add(gradient)),
            _wasm_padding: 0.0,
        }
    }
}
