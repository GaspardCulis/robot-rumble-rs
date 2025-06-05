use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::entities::{planet, satellite::assets as satellite};

/// Responsible for loading all crate's assets, and registering their asset loaders.
pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<satellite::SatelliteConfig>::new(&[
            "satellites.ron",
        ]))
        .add_plugins(RonAssetPlugin::<planet::PlanetsConfig>::new(&[
            "planets.ron",
        ]))
        .add_loading_state(
            LoadingState::new(crate::GameState::AssetLoading)
                .continue_to_state(crate::GameState::MatchMaking)
                .load_collection::<planet::PlanetAssets>()
                .load_collection::<satellite::SatelliteAssets>(),
        );
    }
}
