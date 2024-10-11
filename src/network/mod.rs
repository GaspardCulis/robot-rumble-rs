use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

use lightyear::connection::server::{IoConfig, NetConfig};
use lightyear::prelude::*;
use lightyear::server::config::NetcodeConfig;

mod protocol;

pub const PROTOCOL_ID: u64 = 1;

pub fn shared_config(mode: Mode) -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: Duration::from_millis(40),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / 64.0),
        },
        mode,
    }
}

pub fn net_config() -> NetConfig {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8080);

    let netcode_config = NetcodeConfig::default().with_protocol_id(PROTOCOL_ID);

    let link_conditioner = LinkConditionerConfig {
        incoming_latency: Duration::from_millis(50),
        incoming_jitter: Duration::from_millis(0),
        incoming_loss: 0.00,
    };

    let io_config = IoConfig::from_transport(server::ServerTransport::UdpSocket(addr))
        .with_conditioner(link_conditioner);

    NetConfig::Netcode {
        config: netcode_config,
        io: io_config,
    }
}
