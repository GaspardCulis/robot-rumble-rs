use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;
use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
pub struct BlackHoleRingMaterial {
    #[uniform(0)]
    pub common: CommonMaterial,

    #[uniform(1)]
    pub disk_width: f32,
    #[uniform(1)]
    pub ring_perspective: f32,
    #[uniform(1)]
    pub should_dither: u32, 
    #[uniform(1)]
    pub colors: [LinearRgba; 5],
    #[uniform(1)]
    pub n_colors: i32,

    #[uniform(1)]
    _wasm_padding: Vec2,
}

#[derive(Component, serde::Deserialize, Clone)]
pub struct BlackHoleRingMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Specific to ring
    disk_width: f32,
    ring_perspective: f32,
    should_dither: bool,
    palette: PaletteConfig5,
}

impl Material2d for BlackHoleRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/black_hole_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

impl PlanetMaterial for BlackHoleRingMaterial {
    type Config = BlackHoleRingMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.size = config.size;
        common.octaves = config.octaves;

        Self {
            common,
            disk_width: config.disk_width,
            ring_perspective: config.ring_perspective,
            should_dither: config.should_dither as u32,
            n_colors: 5,
            colors: [
                Srgba::hex(&config.palette[0]).unwrap().into(),
                Srgba::hex(&config.palette[1]).unwrap().into(),
                Srgba::hex(&config.palette[2]).unwrap().into(),
                Srgba::hex(&config.palette[3]).unwrap().into(),
                Srgba::hex(&config.palette[4]).unwrap().into(),
            ],
            _wasm_padding: Vec2::ZERO,
        }
    }
}
