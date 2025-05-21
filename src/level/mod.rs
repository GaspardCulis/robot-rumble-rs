use bevy::prelude::{App, Plugin};

mod map_limit;
mod spawn;

pub struct LevelPlugins;
impl Plugin for LevelPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(map_limit::MapLimitPlugin)
            .add_plugins(spawn::MapSpawnPlugin);
    }
}
