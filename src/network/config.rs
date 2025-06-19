use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_matchbox::matchbox_socket::RtcIceServerConfig;

#[derive(AssetCollection, Resource)]
pub struct NetworkAssets {
    #[asset(path = "config/config.network.ron")]
    pub config: Handle<NetworkConfig>,
}

#[serde_with::serde_as]
#[derive(Asset, TypePath, serde::Deserialize)]
pub struct NetworkConfig {
    /// The domain/IP of the Matchbox signaling server.
    pub matchbox_host: String,
    /// Amount of frames GGRS will delay the inputs for local players.
    pub input_delay: usize,
    // TODO: Figure out what it does
    pub session_fps: usize,
    /// Changes the rate at which GgrsSchedule is called per frame.
    pub schedule_fps: usize,
    // TODO: Figure out what it does
    pub check_distance: usize,
    pub max_prediction_window: usize,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    /// Sets the disconnect timeout. The session will automatically disconnect from a remote peer if it has not received a packet in the timeout window.
    pub disconnect_timeout: std::time::Duration,
    /// Sets the desync detection mode. With desync detection, the session will compare checksums for all peers to detect discrepancies / desyncs between peers.
    pub desync_detection: DesyncDetectionConfig,
    /// Configuration options for an ICE server connection.
    pub ice_server_config: IceServerConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub enum DesyncDetectionConfig {
    On { interval: u32 },
    Off,
}

#[derive(serde::Deserialize, Clone)]
pub struct IceServerConfig {
    urls: Vec<String>,
    username: Option<String>,
    credential: Option<String>,
}

impl Default for NetworkConfig {
    /// Matching bevy_ggrs defaults
    fn default() -> Self {
        Self {
            matchbox_host: "matchbox.gasdev.fr".into(),

            input_delay: 0,
            session_fps: 60,
            schedule_fps: 60,
            check_distance: 2,
            max_prediction_window: 8,
            disconnect_timeout: std::time::Duration::from_secs(2),
            desync_detection: DesyncDetectionConfig::Off,

            ice_server_config: IceServerConfig {
                urls: vec!["turn:gasdev.fr:3478".to_string()],
                username: Some("default".to_string()), // Fixes `ErrNoTurnCredentials`
                credential: Some("default".to_string()), // Same
            },
        }
    }
}

impl From<DesyncDetectionConfig> for bevy_ggrs::ggrs::DesyncDetection {
    fn from(val: DesyncDetectionConfig) -> Self {
        match val {
            DesyncDetectionConfig::On { interval } => {
                bevy_ggrs::ggrs::DesyncDetection::On { interval }
            }
            DesyncDetectionConfig::Off => bevy_ggrs::ggrs::DesyncDetection::Off,
        }
    }
}

impl From<IceServerConfig> for RtcIceServerConfig {
    fn from(value: IceServerConfig) -> Self {
        Self {
            urls: value.urls,
            username: value.username,
            credential: value.credential,
        }
    }
}
