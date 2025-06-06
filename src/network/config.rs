use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

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
    #[allow(dead_code)] // Follow https://github.com/gschup/bevy_ggrs/issues/121
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
}

#[derive(serde::Deserialize, Clone, Copy)]
pub enum DesyncDetectionConfig {
    On { interval: u32 },
    Off,
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
        }
    }
}

impl Into<bevy_ggrs::ggrs::DesyncDetection> for DesyncDetectionConfig {
    fn into(self) -> bevy_ggrs::ggrs::DesyncDetection {
        match self {
            DesyncDetectionConfig::On { interval } => {
                bevy_ggrs::ggrs::DesyncDetection::On { interval }
            }
            DesyncDetectionConfig::Off => bevy_ggrs::ggrs::DesyncDetection::Off,
        }
    }
}
