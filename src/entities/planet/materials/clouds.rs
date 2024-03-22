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
    pub outiline_color: Color,
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
            base_color: Color::rgb(0.875, 0.878, 0.91),
            outiline_color: Color::rgb(0.639, 0.655, 0.761),
            shadow_color: Color::rgb(0.408, 0.435, 0.6),
            shadow_outline_color: Color::rgb(0.251, 0.286, 0.451),
        }
    }
}
