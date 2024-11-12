use bevy::prelude::*;

pub mod core;
pub mod entities;
pub mod network;
pub mod utils;

pub struct CommonPlugins;
impl Plugin for CommonPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(core::CorePlugins)
            .add_plugins(entities::EntitiesPlugin)
            .add_plugins(network::SharedNetworkPlugin);
    }
}
