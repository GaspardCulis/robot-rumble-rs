use crate::core::gravity::{Mass, Passive, Static};
use crate::core::physics::{Position, Rotation, Velocity};
use crate::entities::player::{PlayerAction, PlayerId};
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

        // Gravity
        app.register_component::<Mass>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Passive>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Static>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        // Player
        app.add_plugins(LeafwingInputPlugin::<PlayerAction>::default());

        app.register_component::<PlayerId>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);
    }
}
