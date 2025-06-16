use bevy::prelude::*;

use crate::GameState;

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
    MatchMaking,
    SplitscreenSetup,
}

pub struct MenusPlugin;
impl Plugin for MenusPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((
            home::HomeMenuPlugin,
            splitscreen_setup::SplitscreenSetupPlugin,
        ))
        .add_systems(OnEnter(Screen::MatchMaking), |mut commands: Commands| {
            commands.set_state(GameState::MatchMaking);
        });
    }
}
