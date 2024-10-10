use crate::core::physics::{Position, Rotation, Velocity};
use bevy::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        // Core physics
        app.register_component::<Position>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();

        app.register_component::<Rotation>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_linear_interpolation_fn();

        app.register_component::<Velocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Full);
    }
}
