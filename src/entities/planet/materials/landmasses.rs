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
pub struct LandmassesMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_origin: Vec2,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub land_cutoff: f32,
    #[uniform(1)]
    pub color1: Color,
    #[uniform(1)]
    pub color2: Color,
    #[uniform(1)]
    pub color3: Color,
    #[uniform(1)]
    pub color4: Color,
}

impl Material2d for LandmassesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/landmasses.wgsl".into()
    }
}

impl Default for LandmassesMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial {
                size: 4.292,
                octaves: 6,
                ..Default::default()
            },
            light_origin: Vec2 { x: 0.39, y: 0.39 },
            light_border_1: 0.4,
            light_border_2: 0.5,
            land_cutoff: 0.5,
            color1: Color::hex("#c8d45d").unwrap(),
            color2: Color::hex("#63ab3f").unwrap(),
            color3: Color::hex("#2f5753").unwrap(),
            color4: Color::hex("#283540").unwrap(),
        }
    }
}
