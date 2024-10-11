use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use entities::player::PlayerBundle;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use network::protocol::PROTOCOL_ID;
use network::shared_config;

mod core;
mod entities;
mod network;
mod utils;

const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

#[derive(Resource, Default)]
struct ClientsRecord(HashMap<ClientId, Entity>);

fn init(mut commands: Commands) {
    commands.start_server();
}

fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut clients: ResMut<ClientsRecord>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        let replicate = Replicate::default();
        let entity = commands.spawn((PlayerBundle::new(client_id), replicate));
        clients.0.insert(client_id, entity.id());

        info!("Create entity {:?} for client {:?}", entity.id(), client_id);
    }
}

// FIX: Does not seem to trigger
fn handle_disconnections(
    mut disconnections: EventReader<DisconnectEvent>,
    mut clients: ResMut<ClientsRecord>,
    mut commands: Commands,
) {
    for disconnection in disconnections.read() {
        info!("Client {:?} disconnected", disconnection.client_id);
        if let Some(client_entity) = clients.0.remove(&disconnection.client_id) {
            debug!(
                "Despawning entity {:?} after client {:?} disconnection",
                client_entity, disconnection.client_id
            );
            commands.entity(client_entity).despawn();
        }
    }
}

fn main() {
    let netcode_config = NetcodeConfig::default().with_protocol_id(PROTOCOL_ID);

    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(100),
        incoming_jitter: Duration::from_millis(0),
        incoming_loss: 0.00,
    };

    let io_config = IoConfig::from_transport(server::ServerTransport::UdpSocket(SERVER_ADDR))
        .with_conditioner(link_conditioner);

    let net_config = NetConfig::Netcode {
        config: netcode_config,
        io: io_config,
    };

    let server_config = server::ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![net_config],
        ..Default::default()
    };

    let server_plugin = server::ServerPlugins::new(server_config);

    let mut app = App::new();

    app.add_plugins((MinimalPlugins, StatesPlugin))
        .add_plugins(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            ..default()
        })
        .add_plugins(server_plugin)
        .init_resource::<ClientsRecord>()
        .add_systems(Startup, init)
        .add_systems(Update, (handle_connections, handle_disconnections))
        .run();
}
