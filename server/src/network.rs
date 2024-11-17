use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use rand::Rng as _;
use robot_rumble_common::core::physics;
use robot_rumble_common::entities::player;
use robot_rumble_common::network;

#[derive(Resource, Default)]
struct ClientsRecord(HashMap<ClientId, Entity>);

pub struct ServerNetworkPlugin;
impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let netcode_config =
            server::NetcodeConfig::default().with_protocol_id(network::protocol::PROTOCOL_ID);

        let io_config = server::IoConfig::from_transport(server::ServerTransport::UdpSocket(
            network::SERVER_ADDR,
        ));

        let net_config = server::NetConfig::Netcode {
            config: netcode_config,
            io: io_config,
        };

        let server_config = server::ServerConfig {
            shared: network::shared_config(Mode::Separate),
            net: vec![net_config],
            replication: ReplicationConfig {
                send_interval: network::REPLICATION_SEND_INTERVAL,
                ..Default::default()
            },
            ..Default::default()
        };

        app.add_plugins(server::ServerPlugins::new(server_config))
            .init_resource::<ClientsRecord>()
            .add_systems(Update, (handle_connections, handle_disconnections));
    }
}

fn handle_connections(
    mut connections: EventReader<ConnectEvent>,
    mut clients: ResMut<ClientsRecord>,
    mut commands: Commands,
) {
    for connection in connections.read() {
        let client_id = connection.client_id;
        let replicate = Replicate {
            controlled_by: ControlledBy {
                target: NetworkTarget::Single(client_id),
                ..default()
            },
            group: network::protocol::PLAYER_REPLICATION_GROUP,
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                ..default()
            },
            ..default()
        };
        let entity = commands.spawn((
            player::PlayerBundle::new(
                client_id,
                physics::Position(
                    Vec2::from_angle(rand::thread_rng().gen::<f32>() * 2. * std::f32::consts::PI)
                        * 240.,
                ),
            ),
            ActionState::<player::PlayerAction>::default(),
            replicate,
        ));
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
