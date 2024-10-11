use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use bevy::prelude::*;
use lightyear::prelude::*;

pub mod protocol;

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000);
const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5000);

struct SharedNetworkPlugin;
impl Plugin for SharedNetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(protocol::ProtocolPlugin);
    }
}

pub struct ClientNetworkPlugin;
impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let auth = client::Authentication::Manual {
            server_addr: SERVER_ADDR,
            client_id: 0,
            private_key: Key::default(),
            protocol_id: protocol::PROTOCOL_ID,
        };

        let netcode_config = client::NetcodeConfig::default();

        let io_config =
            client::IoConfig::from_transport(client::ClientTransport::UdpSocket(CLIENT_ADDR));

        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: client::NetConfig::Netcode {
                auth,
                config: netcode_config,
                io: io_config,
            },
            ..default()
        };

        app.add_plugins(client::ClientPlugins::new(client_config))
            .add_plugins(SharedNetworkPlugin);
    }
}

pub struct ServerNetworkPlugin;
impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut App) {
        let netcode_config =
            server::NetcodeConfig::default().with_protocol_id(protocol::PROTOCOL_ID);

        let link_conditioner = LinkConditionerConfig {
            incoming_latency: Duration::from_millis(100),
            incoming_jitter: Duration::from_millis(0),
            incoming_loss: 0.00,
        };

        let io_config =
            server::IoConfig::from_transport(server::ServerTransport::UdpSocket(SERVER_ADDR))
                .with_conditioner(link_conditioner);

        let net_config = server::NetConfig::Netcode {
            config: netcode_config,
            io: io_config,
        };

        let server_config = server::ServerConfig {
            shared: shared_config(Mode::Separate),
            net: vec![net_config],
            ..Default::default()
        };

        app.add_plugins(server::ServerPlugins::new(server_config))
            .add_plugins(SharedNetworkPlugin);
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
