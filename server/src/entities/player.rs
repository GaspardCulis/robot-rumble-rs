use bevy::prelude::*;

use leafwing_input_manager::prelude::ActionState;
use lightyear::prelude::server::*;
use lightyear::prelude::*;

use robot_rumble_common::entities::player::*;

pub struct ServerPlayerPlugin;
impl Plugin for ServerPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, replicate_inputs.before(MainSet::EmitEvents))
            .add_systems(FixedUpdate, server_movement.after(player_physics));
    }
}

fn server_movement(
    mut query: Query<(&ActionState<PlayerAction>, SharedApplyInputsQuery), With<Player>>,
    tick_manager: Res<TickManager>,
) {
    let tick = tick_manager.tick();
    for (action_state, mut saiq) in query.iter_mut() {
        movement_behaviour(action_state, &mut saiq, tick);
    }
}

fn replicate_inputs(
    mut connection: ResMut<ConnectionManager>,
    mut input_events: ResMut<Events<MessageEvent<InputMessage<PlayerAction>>>>,
) {
    for mut event in input_events.drain() {
        let client_id = *event.context();

        // Optional: do some validation on the inputs to check that there's no cheating
        // Inputs for a specific tick should be write *once*. Don't let players change old inputs.

        // rebroadcast the input to other clients
        connection
            .send_message_to_target::<InputChannel, _>(
                &mut event.message,
                NetworkTarget::AllExceptSingle(client_id),
            )
            .unwrap()
    }
}
