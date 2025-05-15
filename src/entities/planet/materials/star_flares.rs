use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::{entities::planet::config::types::*, utils};

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct StarFlaresMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub scale: f32,
    #[uniform(1)]
    pub storm_width: f32,
    #[uniform(1)]
    pub storm_dither_width: f32,
    #[uniform(1)]
    pub circle_amount: f32,
    #[uniform(1)]
    pub circle_scale: f32,
    #[uniform(1)]
    _wasm_padding: Vec3,
    #[texture(2)]
    #[sampler(3)]
    colorscheme_texture: Option<Handle<Image>>,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct StarFlaresMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    scale: f32,
    storm_width: f32,
    storm_dither_width: f32,
    circle_amount: f32,
    circle_scale: f32,
    colorscheme: ColorGradientConfig,
}

impl Material2d for StarFlaresMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/star_flares.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for StarFlaresMaterial {
    type Config = StarFlaresMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.size = config.size;
        common.octaves = config.octaves;
        common.time_speed *= 0.5;

        let gradient = utils::gradient(
            &config.colorscheme.offsets,
            &config
                .colorscheme
                .colors
                .iter()
                .map(|color| Srgba::hex(color).unwrap())
                .collect(),
        );

        Self {
            common,
            scale: config.scale,
            storm_width: config.storm_width,
            storm_dither_width: config.storm_dither_width,
            circle_amount: config.circle_amount,
            circle_scale: config.circle_scale,
            colorscheme_texture: Some(images.add(gradient)),
            _wasm_padding: Default::default(),
        }
    }
}
