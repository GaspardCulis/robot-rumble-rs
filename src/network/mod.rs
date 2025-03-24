use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use inputs::GgrsSessionInput as _;
use leafwing_input_manager::prelude::ActionState;

use crate::entities::player::PlayerAction;

mod inputs;

type SessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<SessionConfig>::default())
            .add_systems(Startup, start_matchbox_socket)
            .add_systems(Update, wait_for_players)
            .add_systems(ReadInputs, read_local_inputs);
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
}

fn read_local_inputs(
    mut commands: Commands,
    query: Query<&ActionState<PlayerAction>>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let action_state = query.single();
        let input = action_state.as_ggrs_session_input();

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<SessionConfig>(local_inputs));
}
