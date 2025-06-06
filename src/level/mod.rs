use bevy::prelude::{App, Plugin};

pub mod limit;
pub mod save;
pub mod spawn;

pub struct LevelPlugins;
impl Plugin for LevelPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(limit::MapLimitPlugin)
            .add_plugins(save::LevelSavePlugin)
            .add_plugins(spawn::MapSpawnPlugin);
    }
}
