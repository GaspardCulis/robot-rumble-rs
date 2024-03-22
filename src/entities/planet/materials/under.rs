use bevy::{
    asset::Asset,
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
            dither_size: 2.0,
            light_border_1: 0.4,
            light_border_2: 0.6,
            color1: Color::hex("#92e8c0").unwrap(),
            color2: Color::hex("#4fa4b8").unwrap(),
            color3: Color::hex("#2c354d").unwrap(),
        }
    }
}
