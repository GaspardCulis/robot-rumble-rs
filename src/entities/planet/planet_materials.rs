use bevy::{
    asset::load_internal_asset,
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
};

pub struct PlanetMaterialsPlugin;

const PLANET_COMMON_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xF750100345124C4BA08A7406DD1CFEC1);

impl Plugin for PlanetMaterialsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PLANET_COMMON_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/planet/common.wgsl"
            ),
            Shader::from_wgsl
        );

        app.add_plugins(Material2dPlugin::<UnderMaterial>::default());
    }
}

#[derive(Asset, TypePath, ShaderType, Debug, Clone)]
pub struct CommonMaterial {
    pub pixels: f32,
    pub rotation: f32,
    pub size: f32,
    pub octaves: i32,
    pub seed: f32,
    pub time_speed: f32,
}

impl Default for CommonMaterial {
    fn default() -> Self {
        Self {
            pixels: 100.0,
            rotation: 0.0,
            size: 50.0,
            octaves: 4,
            seed: 14.0,
            time_speed: 0.2,
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct UnderMaterial {
    #[uniform(0)]
    pub common: CommonMaterial,
    #[uniform(1)]
    pub light_origin: Vec2,
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
        "shaders/planet/planet_under.wgsl".into()
    }
}
