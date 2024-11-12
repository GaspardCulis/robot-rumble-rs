use bevy::prelude::{App, Plugin};

pub mod player;

pub struct ServerEntitiesPlugins;
impl Plugin for ServerEntitiesPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::ServerPlayerPlugin);
    }
}
