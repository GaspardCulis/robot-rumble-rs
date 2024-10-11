use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use bevy::prelude::*;
use lightyear::connection::server::{IoConfig, NetConfig};
use lightyear::prelude::*;
use lightyear::server::config::NetcodeConfig;
use network::{shared_config, PROTOCOL_ID};

mod core;
mod entities;
mod network;
mod utils;

fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    let netcode_config = NetcodeConfig::default().with_protocol_id(PROTOCOL_ID);

    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(100),
        incoming_jitter: Duration::from_millis(0),
        incoming_loss: 0.00,
    };

    let io_config = IoConfig::from_transport(server::ServerTransport::UdpSocket(addr))
        .with_conditioner(link_conditioner);

    let net_config = NetConfig::Netcode {
        config: netcode_config,
        io: io_config,
    };

    let server_config = server::ServerConfig {
        shared: shared_config(Mode::Separate),
        net: vec![net_config],
        ..Default::default()
    };

    let server_plugin = server::ServerPlugins::new(server_config);

    let mut app = App::new();

    app.add_plugins(server_plugin).run();
}
