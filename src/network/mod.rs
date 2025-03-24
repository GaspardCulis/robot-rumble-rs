use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use inputs::GgrsSessionInput as _;
use leafwing_input_manager::prelude::{ActionState, InputMap};

use crate::{
    core::{
        camera,
        physics::{PhysicsSet, Position},
    },
    entities::player::{Player, PlayerAction, PlayerBundle, PlayerSkin},
};

pub mod inputs;

pub type SessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

#[derive(Component, Reflect)]
pub struct LocalPlayer;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<SessionConfig>::default())
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players)
            .add_systems(ReadInputs, read_local_inputs)
            .add_systems(GgrsSchedule, update_remote_inputs.before(PhysicsSet));
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
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

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<SessionConfig>::new()
        .with_num_players(num_players)
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

fn read_local_inputs(
    mut commands: Commands,
    query: Query<(&Player, &ActionState<PlayerAction>), With<LocalPlayer>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    assert_eq!(local_players.0.len(), query.iter().len());
    for (player, action_state) in query.iter() {
        let handle = player.handle;
        let input = action_state.as_ggrs_session_input();

        local_inputs.insert(handle, input);
    }

    commands.insert_resource(LocalInputs::<SessionConfig>(local_inputs));
}

fn update_remote_inputs(
    mut query: Query<(&Player, &mut ActionState<PlayerAction>), Without<LocalPlayer>>,
    inputs: Res<PlayerInputs<SessionConfig>>,
) {
    for (player, mut action_state) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        *action_state = ActionState::<PlayerAction>::from_ggrs_session_input(input);
    }
}
