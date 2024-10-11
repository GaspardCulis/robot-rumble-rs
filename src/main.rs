use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use client::{IoConfig, NetConfig, NetcodeConfig};
use lightyear::prelude::*;
use network::shared_config;

use core::CorePlugins;
use entities::EntitiesPlugins;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod core;
mod entities;
mod network;
mod utils;

fn main() {
    // Network
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let netcode_config = NetcodeConfig::default();

    let io_config = IoConfig::from_transport(client::ClientTransport::UdpSocket(addr));

    let client_config = client::ClientConfig {
        shared: shared_config(Mode::Separate),
        net: NetConfig::Netcode {
            config: netcode_config,
            io: io_config,
            auth: Default::default(),
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
    .add_plugins(EntitiesPlugins);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
