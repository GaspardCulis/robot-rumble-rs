use bevy::{
    asset::Asset,
    math::Vec2,
    reflect::TypePath,
    render::{
        color::Color,
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UnderMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_origin: Vec2,
    #[uniform(1)]
    pub dither_size: f32,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub color1: Color,
    #[uniform(1)]
    pub color2: Color,
    #[uniform(1)]
    pub color3: Color,
}

impl Material2d for UnderMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/under.wgsl".into()
    }
}

impl Default for UnderMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial {
                octaves: 3,
                ..Default::default()
            },
            light_origin: Vec2 { x: 0.39, y: 0.39 },
            dither_size: 2.0,
            light_border_1: 0.4,
            light_border_2: 0.6,
            color1: Color::rgb(0.573, 0.91, 0.753),
            color2: Color::rgb(0.31, 0.643, 0.722),
            color3: Color::rgb(0.173, 0.208, 0.302),
        }
    }
}
