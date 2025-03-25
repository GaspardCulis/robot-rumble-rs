use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    core::{camera, physics, worldgen},
    entities::player::{self, PlayerAction, PlayerBundle, PlayerSkin},
    GameState,
};
use synctest::{
    checksum_position, p2p_mode, spawn_synctest_players, start_synctest_session, synctest_mode,
};

const NUM_PLAYERS: usize = 2;

mod inputs;
mod synctest;

pub type SessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Component, Reflect)]
pub struct LocalPlayer;

#[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
struct SessionSeed(u64);

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<SessionConfig>::default())
            .add_plugins(inputs::NetworkInputsPlugin)
            .rollback_component_with_clone::<physics::Position>()
            .rollback_component_with_clone::<physics::Rotation>()
            .rollback_component_with_clone::<physics::Velocity>()
            .rollback_component_with_clone::<player::InAir>()
            .rollback_component_with_clone::<player::PlayerInputVelocity>()
            .checksum_component::<physics::Position>(checksum_position)
            .add_systems(
                OnEnter(GameState::MatchMaking),
                (
                    start_matchbox_socket.run_if(p2p_mode),
                    start_synctest_session.run_if(synctest_mode),
                ),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    generate_world,
                    spawn_players.run_if(p2p_mode),
                    spawn_synctest_players.run_if(synctest_mode),
                )
                    .chain(),
            )
            .add_systems(
                Update,
                wait_for_players.run_if(in_state(GameState::MatchMaking).and(p2p_mode)),
            );
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = format!("ws://127.0.0.1:3536/extreme_bevy?next={NUM_PLAYERS}");
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    if players.len() < NUM_PLAYERS {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // determine the seed
    let id = socket.id().expect("no peer id assigned").0.as_u64_pair();
    let mut seed = id.0 ^ id.1;
    for peer in socket.connected_peers() {
        let peer_id = peer.0.as_u64_pair();
        seed ^= peer_id.0 ^ peer_id.1;
    }
    commands.insert_resource(SessionSeed(seed));

    let mut session_builder = ggrs::SessionBuilder::<SessionConfig>::new()
        .with_num_players(NUM_PLAYERS)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let channel = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2P(ggrs_session));

    next_state.set(GameState::InGame);
}

fn spawn_players(mut commands: Commands, session: Res<bevy_ggrs::Session<SessionConfig>>) {
    let p2p_session = match &*session {
        Session::P2P(p2_psession) => p2_psession,
        _ => unimplemented!(),
    };

    assert_eq!(p2p_session.local_player_handles().len(), 1);
    for local_handle in p2p_session.local_player_handles() {
        // Local player
        let input_map = InputMap::new([
            // Jump
            (PlayerAction::Jump, KeyCode::Space),
            (PlayerAction::Jump, KeyCode::KeyW),
            // Sneak
            (PlayerAction::Sneak, KeyCode::ShiftLeft),
            (PlayerAction::Sneak, KeyCode::KeyS),
            // Directions
            (PlayerAction::Right, KeyCode::KeyD),
            (PlayerAction::Left, KeyCode::KeyA),
        ]);

        commands
            .spawn((
                input_map,
                LocalPlayer,
                PlayerBundle::new(local_handle, physics::Position(Vec2::ZERO)),
                PlayerSkin("laika".into()),
                camera::CameraFollowTarget,
            ))
            .add_rollback();
    }

    // Remote players
    for remote_handle in p2p_session.remote_player_handles() {
        commands
            .spawn((
                PlayerBundle::new(remote_handle, physics::Position(Vec2::ZERO)),
                PlayerSkin("laika".into()),
            ))
            .add_rollback();
    }
}

fn generate_world(
    mut worldgen_events: EventWriter<worldgen::GenerateWorldEvent>,
    seed: Res<SessionSeed>,
) {
    worldgen_events.send(worldgen::GenerateWorldEvent { seed: seed.0 });
}
