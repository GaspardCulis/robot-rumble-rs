use bevy::prelude::{App, Plugin};

mod limit;
mod spawn;

pub struct LevelPlugins;
impl Plugin for LevelPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(limit::MapLimitPlugin)
            .add_plugins(spawn::MapSpawnPlugin);
    }
}
