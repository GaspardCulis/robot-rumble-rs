use bevy::prelude::Plugin;

use self::{planet::PlanetPlugin, player::PlayerPlugin};

pub mod planet;
pub mod player;

pub enum EntitiesPlugins {
    Client,
    Server,
}

impl Plugin for EntitiesPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        match self {
            EntitiesPlugins::Client => app
                .add_plugins(PlanetPlugin::Client)
                .add_plugins(PlayerPlugin),
            EntitiesPlugins::Server => app.add_plugins(PlanetPlugin::Server),
        };
    }
}
