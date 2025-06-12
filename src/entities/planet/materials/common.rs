use bevy::{asset::Asset, math::Vec2, reflect::Reflect, render::render_resource::ShaderType};

#[derive(serde::Deserialize, Asset, ShaderType, Debug, Clone, Reflect)]
pub struct CommonMaterial {
    #[allow(dead_code)]
    pub pixels: f32,
    #[allow(dead_code)]
    pub rotation: f32,
    #[allow(dead_code)]
    pub size: f32,
    #[allow(dead_code)]
    pub octaves: i32,
    #[allow(dead_code)]
    pub seed: f32,
    #[allow(dead_code)]
    pub time_speed: f32,
    #[allow(dead_code)]
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

impl CommonMaterial {
    pub fn scale(mut self, scale: f32) -> Self {
        self.pixels *= scale;
        self
    }
}
