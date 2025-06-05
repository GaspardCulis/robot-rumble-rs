use bevy::prelude::*;

#[serde_with::serde_as]
#[derive(Asset, TypePath, serde::Deserialize)]
pub struct NetworkConfig {
    pub input_delay: usize,
    pub session_fps: usize,
    #[allow(dead_code)] // Follow https://github.com/gschup/bevy_ggrs/issues/121
    pub schedule_fps: usize,
    pub check_distance: usize,
    pub max_prediction_window: usize,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub disconnect_timeout: std::time::Duration,
    pub desync_detection: DesyncDetectionConfig,
}

#[derive(serde::Deserialize, Clone, Copy)]
pub enum DesyncDetectionConfig {
    On { interval: u32 },
    Off,
}

#[derive(Resource)]
pub struct NetworkConfigHandle(pub Handle<NetworkConfig>);

pub fn load_network_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let network_config = NetworkConfigHandle(asset_server.load("config/network.ron"));
    commands.insert_resource(network_config);
}

impl Default for NetworkConfig {
    /// Matching bevy_ggrs defaults
    fn default() -> Self {
        Self {
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
