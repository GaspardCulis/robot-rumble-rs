use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{entities::planet::config::types::*, utils};

use super::PlanetMaterial;

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct GasLayersMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub bands: f32,
    #[texture(2)]
    #[sampler(3)]
    colorscheme_texture: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    darkcolorscheme_texture: Option<Handle<Image>>,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct GasLayersMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    bands: f32,
    colorscheme: ColorGradientConfig,
    dark_colorscheme: ColorGradientConfig,
}

impl Material2d for GasLayersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/gas_layers.wgsl".into()
    }
}

impl PlanetMaterial for GasLayersMaterial {
    type Config = GasLayersMaterialConfig;

    fn from_config(config: &Self::Config, images: &mut ResMut<Assets<Image>>) -> Self {
        let gradient = utils::gradient(
            &config.colorscheme.offsets,
            &config
                .colorscheme
                .colors
                .iter()
                .map(|color| Srgba::hex(color).unwrap())
                .collect(),
        );

        let dark_gradient = utils::gradient(
            &config.dark_colorscheme.offsets,
            &config
                .dark_colorscheme
                .colors
                .iter()
                .map(|color| Srgba::hex(color).unwrap())
                .collect(),
        );

        Self {
            common: super::CommonMaterial {
                size: config.size,
                octaves: config.octaves,
                light_origin: Vec2::new(-0.1, 0.3),
                ..Default::default()
            },
            bands: config.bands,
            colorscheme_texture: Some(images.add(gradient)),
            darkcolorscheme_texture: Some(images.add(dark_gradient)),
        }
    }
}
