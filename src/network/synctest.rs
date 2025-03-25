use std::hash::{BuildHasher as _, Hash, Hasher as _};

use bevy::{prelude::*, utils::FixedState};
use bevy_ggrs::*;
use leafwing_input_manager::prelude::InputMap;

use crate::{
    core::{camera, physics},
    entities::player::{PlayerAction, PlayerBundle, PlayerSkin},
    network::{LocalPlayer, SessionConfig},
    Args, GameState,
};

const SYNCTEST_NUM_PLAYERS: usize = 2;

pub fn p2p_mode(args: Res<Args>) -> bool {
    !args.synctest
}

pub fn synctest_mode(args: Res<Args>) -> bool {
    args.synctest
}

pub fn checksum_position(position: &physics::Position) -> u64 {
    let mut hasher = FixedState.build_hasher();
    position.x.to_bits().hash(&mut hasher);
    position.y.to_bits().hash(&mut hasher);
    hasher.finish()
}

pub fn start_synctest_session(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("Starting synctest session");

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
    next_state.set(GameState::InGame);
}

pub fn spawn_synctest_players(mut commands: Commands) {
    assert_eq!(SYNCTEST_NUM_PLAYERS, 2);

    let input_map_a = InputMap::new([
        // Jump
        (PlayerAction::Jump, KeyCode::KeyW),
        // Sneak
        (PlayerAction::Sneak, KeyCode::KeyS),
        // Directions
        (PlayerAction::Right, KeyCode::KeyD),
        (PlayerAction::Left, KeyCode::KeyA),
    ]);

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
            LocalPlayer,
            PlayerBundle::new(0, physics::Position(Vec2::ZERO)),
            PlayerSkin("laika".into()),
            camera::CameraFollowTarget,
        ))
        .add_rollback();

    commands
        .spawn((
            input_map_b,
            LocalPlayer,
            PlayerBundle::new(1, physics::Position(Vec2::ZERO)),
            PlayerSkin("laika".into()),
        ))
        .add_rollback();
}
