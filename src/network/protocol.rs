use crate::core::gravity::{Mass, Passive, Static};
use crate::core::physics::{Position, Rotation, Velocity};
use crate::core::worldgen::GenerationSeed;
use crate::entities::planet::{Planet, Radius};
use crate::entities::player::{Player, PlayerAction, PlayerInputVelocity};
use bevy::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

pub struct ProtocolPlugin;

pub const PROTOCOL_ID: u64 = 4;

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

        app.register_component::<Player>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<PlayerInputVelocity>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        // Planet
        app.register_component::<Planet>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        app.register_component::<Radius>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);

        // Worldgen
        app.register_component::<GenerationSeed>(ChannelDirection::ServerToClient)
            .add_prediction(ComponentSyncMode::Once);
    }
}
