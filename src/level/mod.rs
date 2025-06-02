use bevy::prelude::{App, Plugin};

pub mod limit;
pub mod spawn;

pub struct LevelPlugins;
impl Plugin for LevelPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(limit::MapLimitPlugin)
            .add_plugins(spawn::MapSpawnPlugin);
    }
}
