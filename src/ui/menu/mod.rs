use bevy::prelude::*;

mod home;
mod matchmaking_setup;
mod splitscreen_setup;

#[derive(States, Clone, PartialEq, Eq, Debug, Hash, Default)]
pub enum Screen {
    #[default]
    CobwebAssetLoading,
    AssetLoading,
    Home,
    Settings,
    Credits,
    MatchmakingSetup,
    SplitscreenSetup,
    None,
}

pub struct MenusPlugin;
impl Plugin for MenusPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            home::HomeMenuPlugin,
            matchmaking_setup::MatchmakingSetupPlugin,
            splitscreen_setup::SplitscreenSetupPlugin,
        ));
    }
}
