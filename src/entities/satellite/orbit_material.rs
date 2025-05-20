use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::{render::render_resource::*, sprite::Material2d};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct OrbitMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub base_color: LinearRgba,
    #[uniform(0)]
    pub saturation: f32,
    #[uniform(0)]
    pub alpha: f32,
}

impl Material2d for OrbitMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/orbit.wgsl".into()
    }
}
