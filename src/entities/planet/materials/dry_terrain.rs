use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DryTerrainMaterial {
    #[uniform(0)]
    pub common: super::CommonMaterial,
    #[uniform(1)]
    pub dither_size: f32,
    #[uniform(2)]
    pub light_distance_1: f32,
    #[uniform(3)]
    pub light_distance_2: f32,
    #[texture(4)]
    #[sampler(5)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for DryTerrainMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/under.wgsl".into()
    }
}

impl Default for DryTerrainMaterial {
    fn default() -> Self {
        Self {
            common: super::CommonMaterial::default(),
            dither_size: 2.0,
            light_distance_1: 0.4,
            light_distance_2: 0.5,
            color_texture: todo!(),
        }
    }
}
