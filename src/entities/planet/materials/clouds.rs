use bevy::{
    asset::Asset,
    color::{LinearRgba, Srgba},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
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
    pub base_color: LinearRgba,
    #[uniform(1)]
    pub outline_color: LinearRgba,
    #[uniform(1)]
    pub shadow_color: LinearRgba,
    #[uniform(1)]
    pub shadow_outline_color: LinearRgba,
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
            base_color: Srgba::hex("#dfe0e8").unwrap().into(),
            outline_color: Srgba::hex("#a3a7c2").unwrap().into(),
            shadow_color: Srgba::hex("#686f99").unwrap().into(),
            shadow_outline_color: Srgba::hex("#404973").unwrap().into(),
        }
    }
}
