use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{entities::planet::config::types::*, utils};

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct RingMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub ring_width: f32,
    #[uniform(1)]
    pub ring_perspective: f32,
    #[uniform(1)]
    pub scale_rel_to_planet: f32,
    #[uniform(1)]
    _wasm_padding: f32,
    #[texture(2)]
    #[sampler(3)]
    colorscheme_texture: Option<Handle<Image>>,
    #[texture(4)]
    #[sampler(5)]
    darkcolorscheme_texture: Option<Handle<Image>>,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct RingMaterialConfig {
    // Common
    octaves: i32,
    rotation: f32,
    // Material specific
    ring_width: f32,
    ring_perspective: f32,
    scale_rel_to_planet: f32,
    colorscheme: ColorGradientConfig,
    dark_colorscheme: ColorGradientConfig,
}

impl Material2d for RingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/ring.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for RingMaterial {
    type Config = RingMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.octaves = config.octaves;
        common.rotation = config.rotation;

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
            common,
            ring_width: config.ring_width,
            ring_perspective: config.ring_perspective,
            scale_rel_to_planet: config.scale_rel_to_planet,
            colorscheme_texture: Some(images.add(gradient)),
            darkcolorscheme_texture: Some(images.add(dark_gradient)),
            _wasm_padding: 0.0,
        }
    }
}
