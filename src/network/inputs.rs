use crate::{
    core::physics::PhysicsSet,
    entities::player::{Player, PlayerAction},
    network::SessionConfig,
};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ggrs::{GgrsSchedule, LocalInputs, LocalPlayers, PlayerInputs, ReadInputs};
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct NetworkInputs {
    keys: u32, // FIX: Make smaller when https://github.com/gschup/bevy_ggrs#119 is fixed
    pointer_direction: Vec2,
}

const INPUT_UP: u32 = 1 << 0;
const INPUT_DOWN: u32 = 1 << 1;
const INPUT_LEFT: u32 = 1 << 2;
const INPUT_RIGHT: u32 = 1 << 3;
const INPUT_SHOOT: u32 = 1 << 4;
const INPUT_SLOT1: u32 = 1 << 5;
const INPUT_SLOT2: u32 = 1 << 6;
const INPUT_SLOT3: u32 = 1 << 7;
const INPUT_INTERACT: u32 = 1 << 8;
const INPUT_ROPE_EXTEND: u32 = 1 << 9;
const INPUT_ROPE_RETRACT: u32 = 1 << 10;
const INPUT_RELOAD: u32 = 1 << 11;

pub struct NetworkInputsPlugin;
impl Plugin for NetworkInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            ReadInputs,
            (update_local_pointer_direction, read_local_inputs).chain(),
        )
        .add_systems(
            GgrsSchedule,
            update_remote_inputs.before(PhysicsSet::Player),
        );
    }
}

fn read_local_inputs(
    mut commands: Commands,
    query: Query<(&Player, &ActionState<PlayerAction>)>,
    local_players: Res<LocalPlayers>,
) {
    let mut local_inputs = HashMap::new();

    for (player, action_state) in query.iter() {
        let handle = player.handle;

        if !local_players.0.contains(&handle) {
            continue;
        }

        let input = action_state.as_ggrs_session_input();

        local_inputs.insert(handle, input);
    }

    commands.insert_resource(LocalInputs::<SessionConfig>(local_inputs));
}

fn update_remote_inputs(
    mut query: Query<(&Player, &mut ActionState<PlayerAction>)>, // Don't filter out LocalPlayer as we don't want his outputs to fire early
    inputs: Res<PlayerInputs<SessionConfig>>,
) {
    for (player, mut action_state) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        *action_state = ActionState::<PlayerAction>::from_ggrs_session_input(input);
    }
}

pub trait GgrsSessionInput {
    fn as_ggrs_session_input(&self) -> NetworkInputs;

    fn from_ggrs_session_input(input: NetworkInputs) -> Self;
}

impl GgrsSessionInput for ActionState<PlayerAction> {
    fn as_ggrs_session_input(&self) -> NetworkInputs {
        let mut keys = 0;

        for action in self.get_pressed() {
            keys |= match action {
                PlayerAction::Jump => INPUT_UP,
                PlayerAction::Sneak => INPUT_DOWN,
                PlayerAction::Left => INPUT_LEFT,
                PlayerAction::Right => INPUT_RIGHT,
                PlayerAction::Shoot => INPUT_SHOOT,
                PlayerAction::Slot1 => INPUT_SLOT1,
                PlayerAction::Slot2 => INPUT_SLOT2,
                PlayerAction::Slot3 => INPUT_SLOT3,
                PlayerAction::Reload => INPUT_RELOAD,
                PlayerAction::PointerDirection => unimplemented!("Should not get called"),
                PlayerAction::Interact => INPUT_INTERACT,
                PlayerAction::RopeExtend => INPUT_ROPE_EXTEND,
                PlayerAction::RopeRetract => INPUT_ROPE_RETRACT,
            };
        }

        NetworkInputs {
            keys,
            // Avoids rollbacks for other peers as pointer_direction cannot be predicted
            pointer_direction: if keys & INPUT_SHOOT != 0 {
                self.axis_pair(&PlayerAction::PointerDirection)
            } else {
                Vec2::ZERO
            },
        }
    }

    fn from_ggrs_session_input(input: NetworkInputs) -> Self {
        let mut action_state = ActionState::<PlayerAction>::default();

        let keys = input.keys;

        if keys & INPUT_UP != 0 {
            action_state.press(&PlayerAction::Jump);
        }
        if keys & INPUT_DOWN != 0 {
            action_state.press(&PlayerAction::Sneak);
        }
        if keys & INPUT_LEFT != 0 {
            action_state.press(&PlayerAction::Left);
        }
        if keys & INPUT_RIGHT != 0 {
            action_state.press(&PlayerAction::Right);
        }
        if keys & INPUT_SHOOT != 0 {
            action_state.press(&PlayerAction::Shoot);
        }
        if keys & INPUT_SLOT1 != 0 {
            action_state.press(&PlayerAction::Slot1);
        }
        if keys & INPUT_SLOT2 != 0 {
            action_state.press(&PlayerAction::Slot2);
        }
        if keys & INPUT_SLOT3 != 0 {
            action_state.press(&PlayerAction::Slot3);
        }
        if keys & INPUT_RELOAD != 0 {
            action_state.press(&PlayerAction::Reload);
        }
        if keys & INPUT_INTERACT != 0 {
            action_state.press(&PlayerAction::Interact);
        }
        if keys & INPUT_ROPE_EXTEND != 0 {
            action_state.press(&PlayerAction::RopeExtend);
        }
        if keys & INPUT_ROPE_RETRACT != 0 {
            action_state.press(&PlayerAction::RopeRetract);
        }

        action_state.set_axis_pair(&PlayerAction::PointerDirection, input.pointer_direction);

        action_state
    }
}

fn update_local_pointer_direction(
    mut player_query: Query<(&Player, &GlobalTransform, &mut ActionState<PlayerAction>)>,
    windows: Query<&Window>,
    query_view: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    local_players: Res<LocalPlayers>,
) -> Result {
    let window = windows.single()?;
    let (camera, view) = query_view.single()?;
    if let Some(world_position) = window
        .cursor_position()
        .map(|cursor| camera.viewport_to_world_2d(view, cursor).unwrap())
    {
        for (_, player_world_pos, mut action_state) in player_query
            .iter_mut()
            .filter(|(player, _, _)| local_players.0.contains(&player.handle))
        {
            let pointer_direction =
                (world_position - player_world_pos.translation().xy()).normalize();

            action_state.set_axis_pair(&PlayerAction::PointerDirection, pointer_direction);
        }
    } else {
        // Not an error pointer could be out of window
    }

    Ok(())
}
