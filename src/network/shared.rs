use std::time::Duration;

use bevy::prelude::*;
use lightyear::{prelude::*, shared::events::components::MessageEvent};

use crate::core::worldgen;

use super::protocol;

pub struct SharedNetworkPlugin;
impl Plugin for SharedNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(protocol::ProtocolPlugin)
            .add_systems(Update, handle_generate_system_message);
    }
}

pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: Duration::from_millis(40),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        mode,
    }
}

pub fn handle_generate_system_message(
    mut network_events: EventReader<MessageEvent<protocol::GenerateSystemMessage>>,
    mut worldgen_events: EventWriter<worldgen::GenerateWorldEvent>,
) {
    for network_event in network_events.read() {
        worldgen_events.send(worldgen::GenerateWorldEvent {
            seed: network_event.message.seed,
        });
    }
}
