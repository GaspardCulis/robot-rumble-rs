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
pub struct CratersMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border: f32,
    #[uniform(1)]
    pub color1: Color,
    #[uniform(1)]
    pub color2: Color,
}

impl Material2d for CratersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/craters.wgsl".into()
    }
}

impl Default for CratersMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial::default(),
            light_border: 0.5,
            color1: Color::hex("#4c6885").unwrap(),
            color2: Color::hex("#3a3f5e").unwrap(),
        }
    }
}
