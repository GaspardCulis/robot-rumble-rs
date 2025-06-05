use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    core::worldgen,
    entities::{planet, player::weapon::config as weapon, satellite::assets as satellite},
};

/// Responsible for loading all crate's assets, and registering their asset loaders.
pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<worldgen::WorldgenConfig>::new(&[
            "worldgen.ron",
        ]))
        .add_plugins(RonAssetPlugin::<planet::PlanetsConfig>::new(&[
            "planets.ron",
        ]))
        .add_plugins(RonAssetPlugin::<weapon::WeaponsConfig>::new(&[
            "weapons.ron",
        ]))
        .add_plugins(RonAssetPlugin::<satellite::SatelliteConfig>::new(&[
            "satellites.ron",
        ]))
        .add_loading_state(
            LoadingState::new(crate::GameState::AssetLoading)
                .continue_to_state(crate::GameState::MatchMaking)
                .load_collection::<worldgen::WorldgenAssets>()
                .load_collection::<planet::PlanetAssets>()
                .load_collection::<weapon::WeaponsAssets>()
                .load_collection::<satellite::SatelliteAssets>(),
        );
    }
}
