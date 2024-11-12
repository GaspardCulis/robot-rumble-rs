use bevy::{
    log::{Level, LogPlugin},
    prelude::*,
    state::app::StatesPlugin,
};
use lightyear::prelude::server::*;
use rand::Rng as _;

use robot_rumble_common::{core::worldgen, CommonPlugins};

mod entities;
mod network;

fn init(mut commands: Commands, mut worldgen_events: EventWriter<worldgen::GenerateWorldEvent>) {
    commands.start_server();

    worldgen_events.send(worldgen::GenerateWorldEvent {
        seed: rand::thread_rng().gen(),
    });
}

fn main() {
    let mut app = App::new();

    app.add_plugins((MinimalPlugins, StatesPlugin))
        .add_plugins(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            ..default()
        })
        .add_plugins(network::ServerNetworkPlugin)
        .add_plugins(CommonPlugins)
        .add_plugins(entities::ServerEntitiesPlugins)
        .add_systems(Startup, init)
        .run();
}
