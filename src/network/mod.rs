use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use inputs::NetworkInputs;
use leafwing_input_manager::prelude::*;
use rand::Rng as _;

use crate::{
    GameState,
    core::{camera::CameraFollowTarget, collision, physics, worldgen},
    entities::{
        planet,
        player::{self, Player, PlayerAction, weapon},
        projectile,
    },
};
use synctest::{
    checksum_position, handle_ggrs_events, p2p_mode, spawn_synctest_players,
    start_synctest_session, synctest_mode,
};

pub mod inputs;
mod synctest;

pub type SessionConfig = bevy_ggrs::GgrsConfig<NetworkInputs, PeerId>;

#[derive(Resource, Default, Clone, Copy, Debug, Deref, DerefMut)]
pub struct SessionSeed(pub u64);

#[derive(Resource, Default)]
struct StartMatchDelay(Timer);

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GgrsPlugin::<SessionConfig>::default())
            .add_plugins(inputs::NetworkInputsPlugin)
            .rollback_component_with_clone::<physics::Position>()
            .rollback_component_with_clone::<physics::Rotation>()
            .rollback_component_with_clone::<physics::Velocity>()
            .rollback_component_with_clone::<player::PlayerInputVelocity>()
            .rollback_component_with_clone::<player::Weapon>()
            .rollback_component_with_clone::<weapon::Triggered>()
            .rollback_component_with_clone::<weapon::WeaponState>()
            .rollback_component_with_clone::<projectile::Projectile>()
            // Collisions
            .rollback_component_with_clone::<collision::CollisionState<player::Player, planet::Planet>>()
            .rollback_component_with_clone::<collision::CollisionState<projectile::Projectile, planet::Planet>>()
            .rollback_component_with_clone::<collision::CollisionState<projectile::Projectile, player::Player>>()
            .checksum_component::<physics::Position>(checksum_position)
            .add_systems(
                OnEnter(GameState::MatchMaking),
                (
                    start_matchbox_socket.run_if(p2p_mode),
                    start_synctest_session.run_if(synctest_mode),
                ),
            )
            .add_systems(
                OnEnter(GameState::WorldGen),
                (generate_world, spawn_synctest_players.run_if(synctest_mode)).chain(),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (spawn_players, add_local_player_components)
                    .chain()
                    .run_if(p2p_mode),
            )
            .add_systems(
                Update,
                (
                    wait_for_players.run_if(in_state(GameState::MatchMaking).and(p2p_mode)),
                    wait_start_match.run_if(in_state(GameState::WorldGen).and(p2p_mode)),
                    handle_ggrs_events.run_if(in_state(GameState::InGame)),
                ),
            );
    }
}

fn start_matchbox_socket(mut commands: Commands, args: Res<crate::Args>) {
    let room_url = format!(
        "wss://matchbox.gasdev.fr/extreme_bevy?next={}",
        args.players
    );
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url));
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_state: ResMut<NextState<GameState>>,
    args: Res<crate::Args>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    if players.len() < args.players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // determine the seed
    let seed = if args.players > 1 {
        let local_id = socket.id().expect("no peer id assigned").0.as_u64_pair();
        socket
            .connected_peers()
            .map(|peer| peer.0.as_u64_pair())
            .fold(local_id.0 ^ local_id.1, |acc, peer_id| {
                acc ^ (peer_id.0 ^ peer_id.1)
            })
    } else {
        rand::rng().random()
    };
    commands.insert_resource(SessionSeed(seed));
    commands.insert_resource(StartMatchDelay(Timer::from_seconds(0.5, TimerMode::Once)));

    next_state.set(GameState::WorldGen);
}

fn wait_start_match(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_state: ResMut<NextState<GameState>>,
    mut timeout: ResMut<StartMatchDelay>,
    args: Res<crate::Args>,
    time: Res<Time>,
) {
    timeout.0.tick(time.delta());
    if !timeout.0.finished() {
        return;
    }

    let players = socket.players();
    assert_eq!(players.len(), args.players);

    let mut session_builder = ggrs::SessionBuilder::<SessionConfig>::new()
        .with_num_players(args.players)
        .with_input_delay(2);

    if cfg!(feature = "dev_tools") {
        session_builder =
            session_builder.with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 4 });
    }

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

/// Spawn position is handled by level::spawn
fn spawn_players(mut commands: Commands, session: Res<bevy_ggrs::Session<SessionConfig>>) {
    let num_players = match &*session {
        Session::SyncTest(s) => s.num_players(),
        Session::P2P(s) => s.num_players(),
        Session::Spectator(s) => s.num_players(),
    };

    for handle in 0..num_players {
        commands.spawn(Player { handle }).add_rollback();
    }
}

fn generate_world(
    mut worldgen_events: EventWriter<worldgen::GenerateWorldEvent>,
    seed: Res<SessionSeed>,
) {
    worldgen_events.send(worldgen::GenerateWorldEvent { seed: seed.0 });
}

fn add_local_player_components(
    mut commands: Commands,
    query: Query<(Entity, &Player)>,
    session: Res<bevy_ggrs::Session<SessionConfig>>,
) {
    let local_players = match &*session {
        Session::P2P(p2_psession) => p2_psession.local_player_handles(),
        _ => unimplemented!(),
    };

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
        // Slot selection
        (PlayerAction::Slot1, KeyCode::Digit1),
        (PlayerAction::Slot2, KeyCode::Digit2),
        (PlayerAction::Slot3, KeyCode::Digit3),
    ])
    .with(PlayerAction::Shoot, MouseButton::Left)
    .with_dual_axis(PlayerAction::PointerDirection, GamepadStick::RIGHT);

    let local_players_query = query
        .iter()
        .filter(|(_, player)| local_players.contains(&player.handle))
        .collect::<Vec<_>>();

    assert_eq!(local_players_query.len(), 1);
    let (player_entity, _) = local_players_query[0];

    commands
        .entity(player_entity)
        .insert((input_map, CameraFollowTarget));
}
