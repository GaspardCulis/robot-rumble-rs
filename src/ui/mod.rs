use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

mod hud;
mod menus;

pub use menus::Screen;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "img/backgrounds/terrorist-dog.webp")]
    pub background_image: Handle<Image>,
}

pub struct UiPlugins;
impl Plugin for UiPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((hud::HudPlugin, menus::MenusPlugin));
    }
}
