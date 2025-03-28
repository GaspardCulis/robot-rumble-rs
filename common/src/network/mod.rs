use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use lightyear::prelude::*;

pub mod protocol;

pub const REPLICATION_SEND_INTERVAL: Duration = Duration::from_millis(20);
pub const TICK_DURATION: Duration = Duration::from_millis(16);
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 40807);

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
            tick_duration: TICK_DURATION,
        },
        mode,
    }
}
