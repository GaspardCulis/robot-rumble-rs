use core::physics::Position;
use core::worldgen::GenerateWorldEvent;
use core::CorePlugins;
use std::collections::HashMap;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use entities::EntitiesPlugins;
use leafwing_input_manager::action_state::ActionState;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use network::protocol::PLAYER_REPLICATION_GROUP;
use network::ServerNetworkPlugin;

mod core;
mod entities;
mod network;
mod utils;

use entities::player::{PlayerAction, PlayerBundle};
use rand::Rng;

#[derive(Resource, Default)]
struct ClientsRecord(HashMap<ClientId, Entity>);

fn init(mut commands: Commands, mut worldgen_events: EventWriter<GenerateWorldEvent>) {
    commands.start_server();

    worldgen_events.send(GenerateWorldEvent {
        seed: rand::thread_rng().gen(),
    });
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
            group: PLAYER_REPLICATION_GROUP,
            sync: SyncTarget {
                prediction: NetworkTarget::All,
                ..default()
            },
            ..default()
        };
        let entity = commands.spawn((
            PlayerBundle::new(
                client_id,
                Position(
                    Vec2::from_angle(rand::thread_rng().gen::<f32>() * 2. * std::f32::consts::PI)
                        * 240.,
                ),
            ),
            ActionState::<PlayerAction>::default(),
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

fn replicate_inputs(
    mut connection: ResMut<ConnectionManager>,
    mut input_events: ResMut<Events<MessageEvent<InputMessage<PlayerAction>>>>,
) {
    for mut event in input_events.drain() {
        let client_id = *event.context();

        // Optional: do some validation on the inputs to check that there's no cheating
        // Inputs for a specific tick should be write *once*. Don't let players change old inputs.

        // rebroadcast the input to other clients
        connection
            .send_message_to_target::<InputChannel, _>(
                &mut event.message,
                NetworkTarget::AllExceptSingle(client_id),
            )
            .unwrap()
    }
}

fn main() {
    let mut app = App::new();

    app.add_plugins((MinimalPlugins, StatesPlugin, AssetPlugin::default()))
        .add_plugins(LogPlugin {
            level: Level::INFO,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=warn".to_string(),
            ..default()
        })
        .add_plugins(ServerNetworkPlugin)
        .add_plugins(CorePlugins::Server)
        .add_plugins(EntitiesPlugins::Server)
        .init_resource::<ClientsRecord>()
        .add_systems(Startup, init)
        .add_systems(PreUpdate, replicate_inputs.before(MainSet::EmitEvents))
        .add_systems(Update, (handle_connections, handle_disconnections))
        .run();
}
