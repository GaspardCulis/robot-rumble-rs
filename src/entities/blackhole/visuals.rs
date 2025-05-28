use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::materials::CommonMaterial;

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
    pub _wasm_padding: Vec2,
}

impl Material2d for BlackHoleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/black_hole.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}

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
    pub _wasm_padding: Vec2,
}

impl Material2d for BlackHoleRingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/black_hole_ring.wgsl".into()
    }

    fn alpha_mode(&self) -> bevy::sprite::AlphaMode2d {
        bevy::sprite::AlphaMode2d::Blend
    }
}
