use std::hash::{Hash, Hasher as _};

use bevy::prelude::*;
use bevy_ggrs::{ggrs::GgrsEvent, *};
use leafwing_input_manager::prelude::{GamepadStick, InputMap, MouseScrollDirection};
use rand::Rng;

use crate::{
    Args, GameState,
    core::{camera, physics},
    entities::player::{Player, PlayerAction},
    network::{SessionConfig, SessionSeed},
};

const SYNCTEST_NUM_PLAYERS: usize = 2;

pub fn p2p_mode(args: Res<Args>) -> bool {
    !args.synctest
}

pub fn synctest_mode(args: Res<Args>) -> bool {
    args.synctest
}

pub fn checksum_position(position: &physics::Position) -> u64 {
    let mut hasher = checksum_hasher();
    position.x.to_bits().hash(&mut hasher);
    position.y.to_bits().hash(&mut hasher);
    hasher.finish()
}

pub fn start_synctest_session(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Starting synctest session");
    commands.insert_resource(SessionSeed(rand::rng().random()));

    let mut session_builder =
        ggrs::SessionBuilder::<SessionConfig>::new().with_num_players(SYNCTEST_NUM_PLAYERS);

    for i in 0..SYNCTEST_NUM_PLAYERS {
        session_builder = session_builder
            .add_player(ggrs::PlayerType::Local, i)
            .expect("failed to add player");
    }

    let ggrs_session = session_builder
        .start_synctest_session()
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::SyncTest(ggrs_session));

    next_state.set(GameState::WorldGen);
}

pub fn spawn_synctest_players(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    assert_eq!(SYNCTEST_NUM_PLAYERS, 2);

    let input_map_a = InputMap::new([
        // Jump
        (PlayerAction::Jump, KeyCode::KeyW),
        // Sneak
        (PlayerAction::Sneak, KeyCode::KeyS),
        // Directions
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Left, KeyCode::KeyA),
        // Slot selection
        (PlayerAction::Slot1, KeyCode::Digit1),
        (PlayerAction::Slot2, KeyCode::Digit2),
        (PlayerAction::Slot3, KeyCode::Digit3),
        // Reloading
        (PlayerAction::Reload, KeyCode::KeyR),
        // Interaction
        (PlayerAction::Interact, KeyCode::KeyE),
    ])
    .with(PlayerAction::Shoot, MouseButton::Left)
    .with_dual_axis(PlayerAction::PointerDirection, GamepadStick::RIGHT)
    .with(PlayerAction::RopeExtend, MouseScrollDirection::UP)
    .with(PlayerAction::RopeRetract, MouseScrollDirection::DOWN);

    let input_map_b = InputMap::new([
        // Jump
        (PlayerAction::Jump, KeyCode::KeyI),
        // Sneak
        (PlayerAction::Sneak, KeyCode::KeyK),
        // Directions
        (PlayerAction::Right, KeyCode::KeyL),
        (PlayerAction::Left, KeyCode::KeyJ),
    ]);

    commands
        .spawn((
            input_map_a,
            Player { handle: 0 },
            camera::CameraFollowTarget,
        ))
        .add_rollback();

    commands
        .spawn((input_map_b, Player { handle: 1 }))
        .add_rollback();

    next_state.set(GameState::InGame);
}

pub fn handle_ggrs_events(mut session: ResMut<Session<SessionConfig>>) {
    if let Session::P2P(s) = session.as_mut() {
        for event in s.events() {
            match event {
                GgrsEvent::Disconnected { .. } | GgrsEvent::NetworkInterrupted { .. } => {
                    warn!("GGRS event: {event:?}")
                }
                GgrsEvent::DesyncDetected {
                    local_checksum,
                    remote_checksum,
                    frame,
                    ..
                } => {
                    error!(
                        "Desync on frame {frame}. Local checksum: {local_checksum:X}, remote checksum: {remote_checksum:X}"
                    );
                }
                _ => info!("GGRS event: {event:?}"),
            }
        }
    }
}
