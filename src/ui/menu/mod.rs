use bevy::prelude::*;

mod home;
mod splitscreen_setup;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum Screen {
    #[default]
    CobwebAssetLoading,
    AssetLoading,
    Home,
    Settings,
    Credits,
    SplitscreenSetup,
    None,
}

pub struct MenusPlugin;
impl Plugin for MenusPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            home::HomeMenuPlugin,
            splitscreen_setup::SplitscreenSetupPlugin,
        ));
    }
}
