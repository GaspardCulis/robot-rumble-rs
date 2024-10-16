use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::*;

use super::protocol;

pub const REPLICATION_SEND_INTERVAL: Duration = Duration::from_millis(40);

pub struct SharedNetworkPlugin;
impl Plugin for SharedNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(protocol::ProtocolPlugin);
    }
}

pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: REPLICATION_SEND_INTERVAL,
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        mode,
    }
}
