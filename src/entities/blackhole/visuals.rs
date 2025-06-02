use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::materials::CommonMaterial;

// TODO: move to config

// Black hole material values
pub const CORE_SIZE: f32 = 5.0;
pub const CORE_OCTAVES: i32 = 3;
pub const CORE_RADIUS: f32 = 0.5;
pub const CORE_LIGHT_WIDTH: f32 = 0.05;
pub const CORE_SCALE: f32 = 5.0;
pub static CORE_PALETTE: [&str; 3] = ["#000000", "#fef4df", "#ff884d"];

// Ring material values
pub const RING_SIZE: f32 = 6.598;
pub const RING_OCTAVES: i32 = 3;
pub const RING_ROTATION: f32 = 0.766;
pub const RING_DISK_WIDTH: f32 = 0.065;
pub const RING_DISK_PERSPECTIVE: f32 = 14.;
pub static RING_PALETTE: [&str; 5] = ["#000000", "#ffb45c", "#ff8243", "#f25c19", "#fff5cc"];

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
