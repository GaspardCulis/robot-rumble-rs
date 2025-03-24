use crate::entities::player::PlayerAction;
use leafwing_input_manager::prelude::ActionState;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

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
