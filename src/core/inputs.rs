use bevy::prelude::*;
use bevy_ggrs::LocalPlayers;
use leafwing_input_manager::prelude::*;

use crate::entities::player::Player;

/// Threshold for stick inputs to be acknowledged
const GAMEPAD_THRESHOLD: f32 = 0.5;

pub type PlayerActionState = ActionState<PlayerAction>;

#[derive(Actionlike, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect)]
pub enum PlayerAction {
    Jump,
    Sneak,
    Left,
    Right,
    Shoot,
    Slot1,
    Slot2,
    Slot3,
    SlotNext,
    SlotPrev,
    #[actionlike(DualAxis)]
    PointerDirection,
    Reload,
    Interact,
    RopeExtend,
    RopeRetract,
}

pub struct InputsPlugin;
impl Plugin for InputsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, update_local_pointer_direction);
    }
}

fn update_local_pointer_direction(
    mut player_query: Query<(&Player, &GlobalTransform, &mut PlayerActionState)>,
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
        for (_, player_world_pos, mut action_state) in
            player_query.iter_mut().filter(|(player, _, action)| {
                local_players.0.contains(&player.handle)
                    && action.axis_pair(&PlayerAction::PointerDirection) == Vec2::ZERO
            })
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

pub fn default_input_map() -> InputMap<PlayerAction> {
    InputMap::new([
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
        (PlayerAction::SlotNext, KeyCode::Tab),
        // Reload
        (PlayerAction::Reload, KeyCode::KeyR),
        // Interaction
        (PlayerAction::Interact, KeyCode::KeyE),
    ])
    // Mouse
    .with(PlayerAction::Shoot, MouseButton::Left)
    .with(PlayerAction::SlotNext, MouseScrollDirection::UP)
    .with(PlayerAction::SlotPrev, MouseScrollDirection::DOWN)
    .with(PlayerAction::RopeExtend, MouseScrollDirection::UP)
    .with(PlayerAction::RopeRetract, MouseScrollDirection::DOWN)
    // Gamepad
    .with_multiple([
        (PlayerAction::Right, GamepadButton::DPadRight),
        (PlayerAction::Left, GamepadButton::DPadLeft),
        (PlayerAction::Jump, GamepadButton::South),
        (PlayerAction::Shoot, GamepadButton::RightTrigger2),
        (PlayerAction::Reload, GamepadButton::West),
        (PlayerAction::Interact, GamepadButton::East),
        (PlayerAction::SlotNext, GamepadButton::RightTrigger),
        (PlayerAction::SlotPrev, GamepadButton::LeftTrigger),
    ])
    .with_multiple([
        (
            PlayerAction::Right,
            GamepadControlDirection::LEFT_RIGHT.threshold(GAMEPAD_THRESHOLD),
        ),
        (
            PlayerAction::Left,
            GamepadControlDirection::LEFT_LEFT.threshold(GAMEPAD_THRESHOLD),
        ),
    ])
    .with_dual_axis(PlayerAction::PointerDirection, GamepadStick::RIGHT)
}
