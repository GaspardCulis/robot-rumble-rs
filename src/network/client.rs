use bevy::prelude::*;
use lightyear::prelude::*;
use rand::Rng;

use super::{
    protocol,
    shared::{shared_config, SharedNetworkPlugin},
    CLIENT_ADDR, SERVER_ADDR,
};

pub struct ClientNetworkPlugin;
impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let auth = client::Authentication::Manual {
            server_addr: SERVER_ADDR,
            client_id: rand::thread_rng().gen(),
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
