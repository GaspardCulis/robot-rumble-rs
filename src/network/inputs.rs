use std::ops::Deref;

use crate::{
    core::{
        inputs::{PlayerAction, PlayerActionState},
        physics::PhysicsSet,
    },
    entities::player::Player,
    network::SessionConfig,
};
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_ggrs::{GgrsSchedule, LocalInputs, LocalPlayers, PlayerInputs, ReadInputs};
use serde::{Deserialize, Serialize};

/// The list of player actions that gets serialized
/// Actions are expected to be `InputControlKind::Button`
const SERIALIZED_BUTTON_INPUTS: &[PlayerAction] = &[
    PlayerAction::Jump,
    PlayerAction::Sneak,
    PlayerAction::Left,
    PlayerAction::Right,
    PlayerAction::Shoot,
    PlayerAction::Reload,
    PlayerAction::Interact,
    PlayerAction::RopeExtend,
    PlayerAction::RopeRetract,
    PlayerAction::Slot1,
    PlayerAction::Slot2,
    PlayerAction::Slot3,
];

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq)]
// FIX: Make smaller when https://github.com/gschup/bevy_ggrs#119 is fixed
pub struct NetworkInputs {
    keys: u16,
    pointer_direction: AlwaysEqWrapper<Vec2>,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Deref, DerefMut)]
pub struct AlwaysEqWrapper<T>(T);

pub struct NetworkInputsPlugin;
impl Plugin for NetworkInputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(ReadInputs, read_local_inputs).add_systems(
            GgrsSchedule,
            update_remote_inputs.before(PhysicsSet::Player),
        );
    }
}

fn read_local_inputs(
    mut commands: Commands,
    query: Query<(&Player, &PlayerActionState)>,
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
    mut query: Query<(&Player, &mut PlayerActionState)>, // Don't filter out LocalPlayer as we don't want his outputs to fire early
    inputs: Res<PlayerInputs<SessionConfig>>,
) {
    for (player, mut action_state) in query.iter_mut() {
        let (input, _) = inputs[player.handle];
        action_state.set_ggrs_session_input(input);
    }
}

pub trait GgrsSessionInput {
    fn as_ggrs_session_input(&self) -> NetworkInputs;

    fn set_ggrs_session_input(&mut self, input: NetworkInputs);
}

impl GgrsSessionInput for PlayerActionState {
    fn as_ggrs_session_input(&self) -> NetworkInputs {
        let mut keys = 0;

        let buttons = SERIALIZED_BUTTON_INPUTS;

        debug_assert!(buttons.len() < 16);
        for (i, _) in buttons
            .iter()
            .enumerate()
            .filter(|(_, button)| self.pressed(button))
        {
            keys |= 1 << i;
        }

        NetworkInputs {
            keys,
            pointer_direction: self.axis_pair(&PlayerAction::PointerDirection).into(),
        }
    }

    fn set_ggrs_session_input(&mut self, input: NetworkInputs) {
        let keys = input.keys;

        let buttons = SERIALIZED_BUTTON_INPUTS;

        debug_assert!(buttons.len() < 16);
        for (i, action) in buttons.iter().enumerate() {
            self.reset(action);
            if keys & (1 << i) != 0 {
                self.press(action);
            }
        }

        self.set_axis_pair(
            &PlayerAction::PointerDirection,
            *input.pointer_direction.deref(),
        );
    }
}

impl<T> PartialEq for AlwaysEqWrapper<T> {
    /// Tiny hack to not rollback certain kind of inputs even if different
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl<T> From<T> for AlwaysEqWrapper<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
