use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::client::*;
use network::ClientNetworkPlugin;

use core::CorePlugins;
use entities::EntitiesPlugins;

mod core;
mod entities;
mod network;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game".into(),
                    resolution: (1280.0, 720.0).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .add_plugins(ClientNetworkPlugin)
    .add_plugins(CorePlugins::Client)
    .add_plugins(EntitiesPlugins::Client)
    .add_systems(Startup, init);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn init(mut commands: Commands) {
    commands.connect_client();
}
