use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use bevy::prelude::*;
use lightyear::prelude::*;
use rand::Rng;

use robot_rumble_common::{
    core::physics::{Position, Rotation},
    entities::planet::Planet,
    network::{protocol, shared_config, REPLICATION_SEND_INTERVAL},
};

mod config;

pub const INPUT_DELAY_TICKS: u16 = 2;

pub struct ClientNetworkPlugin;
impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut App) {
        let config_str = fs::read_to_string("client/assets/config/network.ron")
            .expect("Failed to read client network config file");
        let config = ron::de::from_str::<config::ClientNetworkConfig>(config_str.as_str())
            .expect("Could not parse the network settings file");

        let auth = client::Authentication::Manual {
            server_addr: SocketAddr::new(config.server_addr.into(), config.server_port),
            client_id: rand::thread_rng().gen(),
            private_key: Key::default(),
            protocol_id: protocol::PROTOCOL_ID,
        };

        let netcode_config = client::NetcodeConfig::default();

        let mut io_config = client::IoConfig::from_transport(client::ClientTransport::UdpSocket(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), config.client_port),
        ));

        if config.conditioner.is_some() && !cfg!(debug_assertions) {
            io_config = io_config.with_conditioner(config.conditioner.unwrap().into());
        }

        let client_config = client::ClientConfig {
            shared: shared_config(Mode::Separate),
            net: client::NetConfig::Netcode {
                auth,
                config: netcode_config,
                io: io_config,
            },
            replication: ReplicationConfig {
                send_interval: REPLICATION_SEND_INTERVAL,
                ..Default::default()
            },
            prediction: client::PredictionConfig {
                minimum_input_delay_ticks: INPUT_DELAY_TICKS,
                maximum_predicted_ticks: 100,
                correction_ticks_factor: 1.5,
                ..Default::default()
            },
            ..default()
        };

        app.add_plugins(client::ClientPlugins::new(client_config))
            .add_plugins(client::VisualInterpolationPlugin::<Position>::default())
            .add_plugins(client::VisualInterpolationPlugin::<Rotation>::default())
            .observe(add_visual_interpolation_components::<Position>)
            .observe(add_visual_interpolation_components::<Rotation>);
    }
}

fn add_visual_interpolation_components<T: Component>(
    trigger: Trigger<OnAdd, T>,
    q: Query<Entity, (With<T>, Without<Planet>, With<client::Predicted>)>,
    mut commands: Commands,
) {
    if !q.contains(trigger.entity()) {
        return;
    }
    debug!("Adding visual interp component to {:?}", trigger.entity());
    commands
        .entity(trigger.entity())
        .insert(client::VisualInterpolateStatus::<T> {
            trigger_change_detection: true,
            ..default()
        });
}
