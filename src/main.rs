use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use lightyear::prelude::client::*;
use lightyear::prelude::*;
use network::{shared_config, PROTOCOL_ID};

use core::CorePlugins;
use entities::EntitiesPlugins;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod core;
mod entities;
mod network;
mod utils;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);
const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

fn main() {
    // Network
    let auth = Authentication::Manual {
        server_addr: SERVER_ADDR,
        client_id: 0,
        private_key: Key::default(),
        protocol_id: PROTOCOL_ID,
    };

    let netcode_config = NetcodeConfig::default();

    let io_config = IoConfig::from_transport(client::ClientTransport::UdpSocket(CLIENT_ADDR));

    let client_config = client::ClientConfig {
        shared: shared_config(Mode::Separate),
        net: NetConfig::Netcode {
            auth,
            config: netcode_config,
            io: io_config,
        },
        ..default()
    };

    let client_plugins = client::ClientPlugins::new(client_config);

    // Build the app
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
    .add_plugins(client_plugins)
    .add_plugins(CorePlugins)
    .add_plugins(EntitiesPlugins)
    .add_systems(Startup, init);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}

fn init(mut commands: Commands) {
    commands.connect_client();
}
