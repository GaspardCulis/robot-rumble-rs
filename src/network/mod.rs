use bevy::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;
use leafwing_input_manager::prelude::InputMap;
use rand::{seq::SliceRandom, Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::{
    core::{camera::CameraFollowTarget, physics, worldgen},
    entities::{
        planet::{Planet, Radius},
        player::{self, Player, PlayerAction, PlayerBundle, PlayerSkin, PLAYER_RADIUS},
    },
    GameState,
};
use synctest::{
    checksum_position, handle_ggrs_events, p2p_mode, spawn_synctest_players,
    start_synctest_session, synctest_mode,
};

const NUM_PLAYERS: usize = 2;

pub mod inputs;
mod synctest;

pub type SessionConfig = bevy_ggrs::GgrsConfig<u8, PeerId>;

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
                OnEnter(GameState::WorldGen),
                (
                    generate_world,
                    prespawn_players.run_if(p2p_mode),
                    spawn_synctest_players.run_if(synctest_mode),
                ),
            )
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    position_players,
                    add_local_player_components.run_if(p2p_mode),
                )
                    .chain(),
            )
            .add_systems(
                GgrsSchedule,
                tick_worldgen_cooldown.run_if(in_state(GameState::WorldGen)),
            )
            .add_systems(
                Update,
                (
                    wait_for_players.run_if(in_state(GameState::MatchMaking).and(p2p_mode)),
                    handle_ggrs_events.run_if(in_state(GameState::InGame)),
                ),
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
        .with_desync_detection_mode(ggrs::DesyncDetection::On { interval: 4 })
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

    next_state.set(GameState::WorldGen);
}

/// Prespawn all players regardless if they are local or not. Positioning will be done later
fn prespawn_players(mut commands: Commands, session: Res<bevy_ggrs::Session<SessionConfig>>) {
    let num_players = match &*session {
        Session::SyncTest(s) => s.num_players(),
        Session::P2P(s) => s.num_players(),
        Session::Spectator(s) => s.num_players(),
    };

    for handle in 0..num_players {
        commands
            .spawn((
                PlayerBundle::new(handle, physics::Position(Vec2::ZERO)),
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

fn position_players(
    mut players_query: Query<
        (&Player, &mut physics::Position, &mut physics::Velocity),
        (With<Player>, Without<Planet>),
    >,
    planets_query: Query<(&physics::Position, &Radius), With<Planet>>,
    seed: Res<SessionSeed>,
) {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.0);

    let mut planets = planets_query.iter().collect::<Vec<_>>();
    planets.sort_by(|(a_pos, _), (b_pos, _)| {
        a_pos.length_squared().total_cmp(&b_pos.length_squared())
    });
    planets.shuffle(&mut rng);

    for (player, mut player_position, mut player_velocity) in players_query.iter_mut() {
        player_velocity.0 = Vec2::ZERO;

        let (spawn_planet_pos, spawn_planet_radius) = planets
            .get(player.handle)
            .expect("Should have more planets than players");

        let random_direction = Vec2::from_angle(rng.random::<f32>() * 2. * std::f32::consts::PI);
        player_position.0 =
            spawn_planet_pos.0 + random_direction * (spawn_planet_radius.0 as f32 + PLAYER_RADIUS);
        player_position.0 = Vec2::ZERO + random_direction * 300.0;
    }
}

fn add_local_player_components(
    mut commands: Commands,
    query: Query<(Entity, &Player)>,
    local_players: Res<LocalPlayers>,
) {
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

    let local_players_query = query
        .iter()
        .filter(|(_, player)| local_players.0.contains(&player.handle))
        .collect::<Vec<_>>();

    assert_eq!(local_players_query.len(), 1);
    let (player_entity, _) = local_players_query[0];

    commands
        .entity(player_entity)
        .insert((input_map, CameraFollowTarget));
}

fn tick_worldgen_cooldown(
    frame: Res<RollbackFrameCount>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if frame.0 > 5 {
        next_state.set(GameState::InGame);
    }
}
