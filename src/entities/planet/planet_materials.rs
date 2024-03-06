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

        app.add_plugins(Material2dPlugin::<UnderMaterial>::default())
            .add_plugins(Material2dPlugin::<LandmassesMaterial>::default());
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
            size: 5.0,
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
        "shaders/planet/under.wgsl".into()
    }
}

impl Default for UnderMaterial {
    fn default() -> Self {
        Self {
            common: CommonMaterial {
                octaves: 3,
                ..Default::default()
            },
            light_origin: Vec2 { x: 0.39, y: 0.39 },
            dither_size: 2.0,
            light_border_1: 0.4,
            light_border_2: 0.6,
            color1: Color::rgb(0.573, 0.91, 0.753),
            color2: Color::rgb(0.31, 0.643, 0.722),
            color3: Color::rgb(0.173, 0.208, 0.302),
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct LandmassesMaterial {
    #[uniform(0)]
    pub common: CommonMaterial,
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
            common: CommonMaterial {
                size: 4.292,
                octaves: 6,
                ..Default::default()
            },
            light_origin: Vec2 { x: 0.39, y: 0.39 },
            light_border_1: 0.4,
            light_border_2: 0.5,
            land_cutoff: 0.5,
            color1: Color::rgb(0.784, 0.831, 0.365),
            color2: Color::rgb(0.388, 0.671, 0.247),
            color3: Color::rgb(0.184, 0.341, 0.325),
            color4: Color::rgb(0.157, 0.208, 0.251),
        }
    }
}
