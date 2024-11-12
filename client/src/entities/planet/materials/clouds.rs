use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

use crate::entities::planet::config::types::*;

use super::{CommonMaterial, PlanetMaterial};

#[derive(Asset, Reflect, AsBindGroup, Debug, Clone)]
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

#[derive(Component, serde::Deserialize, Clone)]
pub struct CloudsMaterialConfig {
    // Common
    size: f32,
    octaves: i32,
    // Material specific
    cloud_cover: f32,
    stretch: f32,
    cloud_curve: f32,
    light_border_1: f32,
    light_border_2: f32,
    palette: PaletteConfig4,
}

impl Material2d for CloudsMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet/clouds.wgsl".into()
    }
}

impl PlanetMaterial for CloudsMaterial {
    type Config = CloudsMaterialConfig;

    fn from_config(
        mut common: CommonMaterial,
        config: &Self::Config,
        _: &mut ResMut<Assets<Image>>,
    ) -> Self {
        common.octaves = config.octaves;
        common.size = config.size;

        Self {
            common,
            cloud_cover: config.cloud_cover,
            stretch: config.stretch,
            cloud_curve: config.cloud_curve,
            light_border_1: config.light_border_1,
            light_border_2: config.light_border_2,
            base_color: Srgba::hex(&config.palette[0]).unwrap().into(),
            outline_color: Srgba::hex(&config.palette[1]).unwrap().into(),
            shadow_color: Srgba::hex(&config.palette[2]).unwrap().into(),
            shadow_outline_color: Srgba::hex(&config.palette[3]).unwrap().into(),
        }
    }
}
