use crate::{
    core::physics::PhysicsSet,
    entities::player::{Player, PlayerAction},
    network::SessionConfig,
};
use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{GgrsSchedule, LocalInputs, LocalPlayers, PlayerInputs, ReadInputs};
use leafwing_input_manager::prelude::ActionState;

use super::LocalPlayer;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

pub struct NetworkInputsPlugin;
impl Plugin for NetworkInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ReadInputs, read_local_inputs)
            .add_systems(GgrsSchedule, update_remote_inputs.before(PhysicsSet));
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

pub trait GgrsSessionInput {
    fn as_ggrs_session_input(&self) -> u8;

    fn from_ggrs_session_input(input: u8) -> Self;
}

impl GgrsSessionInput for ActionState<PlayerAction> {
    fn as_ggrs_session_input(&self) -> u8 {
        let mut input = 0u8;

        for action in self.get_pressed() {
            input |= match action {
                PlayerAction::Jump => INPUT_UP,
                PlayerAction::Sneak => INPUT_DOWN,
                PlayerAction::Left => INPUT_LEFT,
                PlayerAction::Right => INPUT_RIGHT,
            };
        }

        input
    }

    fn from_ggrs_session_input(input: u8) -> Self {
        let mut action_state = ActionState::<PlayerAction>::default();

        if input & INPUT_UP != 0 {
            action_state.press(&PlayerAction::Jump);
        }
        if input & INPUT_DOWN != 0 {
            action_state.press(&PlayerAction::Sneak);
        }
        if input & INPUT_LEFT != 0 {
            action_state.press(&PlayerAction::Left);
        }
        if input & INPUT_RIGHT != 0 {
            action_state.press(&PlayerAction::Right);
        }

        action_state
    }
}
