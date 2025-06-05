use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::entities::satellite::assets as satellite_assets;

/// Responsible for loading all crate's assets, and registering their asset loaders.
pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<satellite_assets::SatelliteConfig>::new(&[
            "ron",
        ]))
        .add_loading_state(
            LoadingState::new(crate::GameState::AssetLoading)
                .continue_to_state(crate::GameState::MatchMaking)
                .load_collection::<satellite_assets::SatelliteAssets>(),
        );
    }
}
