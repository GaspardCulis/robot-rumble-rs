use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_cobweb_ui::prelude::*;

mod hud;
mod menu;

pub use menu::Screen;

#[derive(AssetCollection, Resource)]
pub struct UIAssets {
    #[asset(path = "img/backgrounds/terrorist-dog.webp")]
    pub background_image: Handle<Image>,
}

pub struct UiPlugins;
impl Plugin for UiPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(CobwebUiPlugin)
            .add_plugins((hud::HUDPlugin, menu::MenusPlugin))
            .load("ui/main.cob")
            .add_systems(
                OnEnter(LoadState::Done),
                |mut next: ResMut<NextState<Screen>>| {
                    next.set(Screen::AssetLoading);
                },
            );
    }
}
