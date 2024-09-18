use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{entities::planet::config::types::*, utils};

use super::PlanetMaterial;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DryTerrainMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub dither_size: f32,
    #[uniform(2)]
    pub light_distance_1: f32,
    #[uniform(3)]
    pub light_distance_2: f32,
    #[texture(4)]
    #[sampler(5)]
    color_texture: Option<Handle<Image>>,
}

#[derive(Component, serde::Deserialize)]
pub struct DryTerrainMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    dither_size: f32,
    light_distance_1: f32,
    light_distance_2: f32,
    colors: ColorGradientConfig,
}

impl Material2d for DryTerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/under.wgsl".into()
    }
}

impl PlanetMaterial for DryTerrainMaterial {
    type Config = DryTerrainMaterialConfig;

    fn from_config(config: &Self::Config, images: &mut ResMut<Assets<Image>>) -> Self {
        let gradient = utils::gradient(
            &config.colors.offsets,
            &config
                .colors
                .colors
                .iter()
                .map(|color| Srgba::hex(color).unwrap())
                .collect(),
        );

        Self {
            common: super::CommonMaterial {
                size: config.size,
                octaves: config.octaves,
                ..Default::default()
            },
            dither_size: config.dither_size,
            light_distance_1: config.light_distance_1,
            light_distance_2: config.light_distance_2,
            color_texture: Some(images.add(gradient)),
        }
    }
}
