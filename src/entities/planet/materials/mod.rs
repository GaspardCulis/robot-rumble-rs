use bevy::{asset::load_internal_asset, prelude::*, sprite::Material2dPlugin};

mod clouds;
mod common;
mod landmasses;
mod under;
pub use clouds::CloudsMaterial;
pub use common::CommonMaterial;
pub use landmasses::LandmassesMaterial;
pub use under::UnderMaterial;

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

        app.add_plugins(Material2dPlugin::<CloudsMaterial>::default())
            .add_plugins(Material2dPlugin::<LandmassesMaterial>::default())
            .add_plugins(Material2dPlugin::<UnderMaterial>::default());
    }
}
