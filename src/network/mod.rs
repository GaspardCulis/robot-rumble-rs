use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    core::{camera, physics::Position},
    entities::player::{PlayerAction, PlayerBundle, PlayerSkin},
};

const NUM_PLAYERS: usize = 2;

mod inputs;

pub type SessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Component, Reflect)]
pub struct LocalPlayer;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<SessionConfig>::default())
            .add_plugins(inputs::NetworkInputsPlugin)
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = format!("ws://127.0.0.1:3536/extreme_bevy?next={NUM_PLAYERS}");
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket>) {
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

    // Spawn players
    let system_id = commands.register_system(spawn_players);
    commands.run_system(system_id);
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

        commands.spawn((
            input_map,
            LocalPlayer,
            PlayerBundle::new(local_handle, Position(Vec2::ZERO)),
            PlayerSkin("laika".into()),
            camera::CameraFollowTarget,
        ));
    }

    // Remote players
    for remote_handle in p2p_session.remote_player_handles() {
        commands.spawn((
            PlayerBundle::new(remote_handle, Position(Vec2::ZERO)),
            PlayerSkin("laika".into()),
        ));
    }
}
