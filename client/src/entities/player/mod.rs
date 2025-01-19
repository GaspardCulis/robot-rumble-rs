use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;

use lightyear::prelude::client::*;
use lightyear::prelude::*;
use lightyear::{
    inputs::leafwing::input_buffer::InputBuffer, prelude::client::Predicted,
    shared::replication::components::Controlled,
};
use robot_rumble_common::entities::player::*;

use crate::core::camera::CameraFollowTarget;
use crate::network;

mod animation;
mod skin;

pub struct ClientPlayerPlugin;
impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(skin::SkinPlugin)
            .add_plugins(animation::PlayerAnimationPlugin)
            .add_systems(
                PreUpdate,
                handle_new_player
                    .after(MainSet::Receive)
                    .before(PredictionSet::SpawnPrediction),
            )
            .add_systems(
                PreUpdate,
                handle_shoot_click.after(InputManagerSystem::ManualControl),
            )
            .add_systems(
                FixedUpdate,
                client_movement.in_set(PlayerSet).after(player_physics),
            );
    }
}

fn handle_new_player(
    mut commands: Commands,
    player_query: Query<(Entity, Has<Controlled>), (Added<Predicted>, With<Player>)>,
) {
    for (player_entity, is_controlled) in player_query.iter() {
        let mut player_commands = commands.entity(player_entity);

        if is_controlled {
            info!("Own player replicated to us, adding inputmap to {player_entity:?}");
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
            ])
            .with_dual_axis(PlayerAction::Shoot, GamepadStick::RIGHT);

            player_commands.insert((input_map, CameraFollowTarget));
        } else {
            info!("Remote player replicated to us: {player_entity:?}");
        }

        player_commands.insert(Name::new(format!(
            "{} Player",
            if is_controlled {
                "Controlled"
            } else {
                "Remote"
            }
        )));
    }
}

fn client_movement(
    mut query: Query<
        (
            &ActionState<PlayerAction>,
            &InputBuffer<PlayerAction>,
            SharedApplyInputsQuery,
        ),
        (With<Player>, With<Predicted>),
    >,
    tick_manager: Res<TickManager>,
    rollback: Option<Res<Rollback>>,
) {
    // max number of stale inputs to predict before default inputs used
    const MAX_STALE_TICKS: u16 = network::INPUT_DELAY_TICKS;
    // get the tick, even if during rollback
    let tick = rollback
        .as_ref()
        .map(|rb| tick_manager.tick_or_rollback_tick(rb))
        .unwrap_or(tick_manager.tick());

    for (action_state, input_buffer, mut saiq) in query.iter_mut() {
        // is the current ActionState for real?
        if input_buffer.get(tick).is_some() {
            // Got an exact input for this tick, staleness = 0, the happy path.
            movement_behaviour(action_state, &mut saiq, tick);
            continue;
        }

        // if the true input is missing, this will be leftover from a previous tick, or the default().
        if let Some((prev_tick, prev_input)) = input_buffer.get_last_with_tick() {
            let staleness = (tick - prev_tick).max(0) as u16;
            if staleness > MAX_STALE_TICKS {
                // input too stale, apply default input (ie, nothing pressed)
                movement_behaviour(&ActionState::default(), &mut saiq, tick);
            } else {
                // apply a stale input within our acceptable threshold.
                // we could use the staleness to decay movement forces as desired.
                movement_behaviour(prev_input, &mut saiq, tick);
            }
        } else {
            // no inputs in the buffer yet, can happen during initial connection.
            // apply the default input (ie, nothing pressed)
            movement_behaviour(action_state, &mut saiq, tick);
        }
    }
}

fn handle_shoot_click(
    windows: Query<&Window>,
    query_view: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<
        (&GlobalTransform, &mut ActionState<PlayerAction>),
        (With<Controlled>, With<Player>),
    >,
) {
    let window = windows.single();
    let (camera, view) = query_view.single();
    let Ok((player_world_pos, mut action_state)) = player_query.get_single_mut() else {
        return;
    };
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(view, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse.pressed(MouseButton::Left) {
            let shoot_direction =
                (world_position - player_world_pos.translation().xy()).normalize();
            action_state.set_axis_pair(&PlayerAction::Shoot, shoot_direction);
        }
    }
}
