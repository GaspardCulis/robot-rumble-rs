use bevy::prelude::{App, Plugin};

pub mod planet;
pub mod player;

pub struct EntitiesPlugins;
impl Plugin for EntitiesPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(planet::PlanetPlugin)
            .add_plugins(player::PlayerPlugin);
    }
}
