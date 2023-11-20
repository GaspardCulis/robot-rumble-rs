use bevy::prelude::Plugin;

use self::player::PlayerPlugin;

pub mod planet;
pub mod player;

pub struct EntitiesPlugins;

impl Plugin for EntitiesPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(PlayerPlugin);
    }
}
