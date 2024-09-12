use bevy::{
    asset::Asset,
    color::{LinearRgba, Srgba},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LandmassesMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub light_border_1: f32,
    #[uniform(1)]
    pub light_border_2: f32,
    #[uniform(1)]
    pub land_cutoff: f32,
    #[uniform(1)]
    pub color1: LinearRgba,
    #[uniform(1)]
    pub color2: LinearRgba,
    #[uniform(1)]
    pub color3: LinearRgba,
    #[uniform(1)]
    pub color4: LinearRgba,
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
            light_border_1: 0.4,
            light_border_2: 0.5,
            land_cutoff: 0.5,
            color1: Srgba::hex("#c8d45d").unwrap().into(),
            color2: Srgba::hex("#63ab3f").unwrap().into(),
            color3: Srgba::hex("#2f5753").unwrap().into(),
            color4: Srgba::hex("#283540").unwrap().into(),
        }
    }
}
