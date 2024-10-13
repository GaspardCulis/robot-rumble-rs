use std::time::Duration;

use bevy::prelude::*;
use lightyear::prelude::*;

use super::{
    protocol,
    shared::{shared_config, SharedNetworkPlugin},
    SERVER_ADDR,
};

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
