use bevy::{
    asset::Asset,
    color::{LinearRgba, Srgba},
    math::Vec2,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CratersMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border: f32,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(1)]
    pub color2: LinearRgba,
}

impl Material2d for CratersMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/craters.wgsl".into()
    }
}

impl Default for CratersMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial {
                light_origin: Vec2::new(0.25, 0.25),
                ..Default::default()
            },
            light_border: 0.5,
            color1: Srgba::hex("#4c6885").unwrap().into(),
            color2: Srgba::hex("#3a3f5e").unwrap().into(),
        }
    }
}
