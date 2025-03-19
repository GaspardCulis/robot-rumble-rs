use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::Rng as _;

use bevy_matchbox::prelude::*;

mod core;
mod entities;
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
    .add_plugins(core::CorePlugins)
    .add_plugins(entities::EntitiesPlugins)
    .add_systems(Startup, start_matchbox_socket)
    .add_systems(Update, wait_for_players);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn init(mut worldgen_events: EventWriter<core::worldgen::GenerateWorldEvent>) {
    worldgen_events.send(core::worldgen::GenerateWorldEvent {
        seed: rand::thread_rng().gen(),
    });
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

fn wait_for_players(mut socket: ResMut<MatchboxSocket>) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");
    // TODO
}
