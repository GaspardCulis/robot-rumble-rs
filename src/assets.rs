use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    core::worldgen,
    entities::{
        planet,
        player::{skin as player_skin, weapon::config as weapon},
        projectile::config as projectiles,
        satellite::assets as satellite,
    },
    ui,
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
        .init_asset::<player_skin::Skin>()
        .add_plugins(RonAssetPlugin::<player_skin::SkinConfig>::new(&[
            "skin.ron",
        ]))
        .add_plugins(RonAssetPlugin::<weapon::WeaponsConfig>::new(&[
            "weapons.ron",
        ]))
        .add_plugins(RonAssetPlugin::<projectiles::ProjectilesConfig>::new(&[
            "projectiles.ron",
        ]))
        .add_plugins(RonAssetPlugin::<satellite::SatelliteConfig>::new(&[
            "satellites.ron",
        ]))
        .add_loading_state(
            LoadingState::new(crate::ui::Screen::AssetLoading)
                .continue_to_state(crate::ui::Screen::Home)
                .load_collection::<worldgen::WorldgenAssets>()
                .load_collection::<planet::PlanetAssets>()
                .load_collection::<player_skin::SkinConfigAssets>()
                .finally_init_resource::<player_skin::SkinAssets>()
                .load_collection::<weapon::WeaponsAssets>()
                .load_collection::<projectiles::ProjectilesAssets>()
                .load_collection::<satellite::SatelliteAssets>()
                .load_collection::<ui::UiAssets>(),
        );
    }
}
