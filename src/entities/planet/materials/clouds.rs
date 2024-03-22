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
pub struct CloudsMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub cloud_cover: f32,
    #[uniform(1)]
    pub stretch: f32,
    #[uniform(1)]
    pub cloud_curve: f32,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub light_origin: Vec2,
    #[uniform(1)]
    pub base_color: Color,
    #[uniform(1)]
    pub outline_color: Color,
    #[uniform(1)]
    pub shadow_color: Color,
    #[uniform(1)]
    pub shadow_outline_color: Color,
}

impl Material2d for CloudsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/clouds.wgsl".into()
    }
}

impl Default for CloudsMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial {
                size: 7.745,
                octaves: 2,
                ..Default::default()
            },
            cloud_cover: 0.415,
            stretch: 2.0,
            cloud_curve: 1.3,
            light_border_1: 0.5,
            light_border_2: 0.6,
            light_origin: Vec2 { x: 0.39, y: 0.39 },
            base_color: Color::hex("#dfe0e8").unwrap(),
            outline_color: Color::hex("#a3a7c2").unwrap(),
            shadow_color: Color::hex("#686f99").unwrap(),
            shadow_outline_color: Color::hex("#404973").unwrap(),
        }
    }
}
