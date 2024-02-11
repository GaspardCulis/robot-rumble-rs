use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin},
};

pub struct PlanetMaterialsPlugin;

impl Plugin for PlanetMaterialsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<UnderMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UnderMaterial {
    #[uniform(0)]
    pub pixels: f32,
    #[uniform(0)]
    pub rotation: f32,
    #[uniform(0)]
    pub light_origin: Vec2,
    #[uniform(0)]
    pub time_speed: f32,
    #[uniform(0)]
    pub dither_size: f32,
    #[uniform(0)]
    pub light_border_1: f32,
    #[uniform(0)]
    pub light_border_2: f32,
    #[uniform(0)]
    pub color1: Color,
    #[uniform(0)]
    pub color2: Color,
    #[uniform(0)]
    pub color3: Color,
    #[uniform(0)]
    pub size: f32,
    #[uniform(0)]
    pub octaves: i32,
    #[uniform(0)]
    pub seed: f32,
}

impl Material2d for UnderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/planet_under.wgsl".into()
    }
}
