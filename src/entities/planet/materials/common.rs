use bevy::{asset::Asset, math::Vec2, reflect::TypePath, render::render_resource::ShaderType};

#[derive(Asset, TypePath, ShaderType, Debug, Clone)]
pub struct CommonMaterial {
    pub pixels: f32,
    pub rotation: f32,
    pub size: f32,
    pub octaves: i32,
    pub seed: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
}

impl Default for CommonMaterial {
    fn default() -> Self {
        Self {
            pixels: 100.0,
            rotation: 0.0,
            size: 5.0,
            octaves: 4,
            seed: 14.0,
            time_speed: 0.2,
            light_origin: Vec2 { x: 0.39, y: 0.39 },
        }
    }
}
