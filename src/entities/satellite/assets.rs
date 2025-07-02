use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct SatelliteAssets {
    #[asset(path = "config/config.satellites.ron")]
    pub config: Handle<SatelliteConfig>,
    #[asset(path = "img/satellites/working_graviton.png")]
    pub working_graviton: Handle<Image>,
    #[asset(path = "img/satellites/destroyed_graviton.png")]
    pub destroyed_graviton: Handle<Image>,
    #[asset(path = "img/satellites/working_bumper.png")]
    pub working_bumper: Handle<Image>,
    #[asset(path = "img/satellites/working_grabber.png")]
    pub working_grabber: Handle<Image>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct SatelliteConfig {
    pub bumper: BumperConfig,
    pub grabber: GrabberConfig,
    pub slingshot: SlingshotConfig,
}

#[derive(serde::Deserialize)]
pub struct BumperConfig {
    /// Interaction radius
    pub radius: f32,
    /// Multiplier by which player speed is multiplied on interaction
    pub multiplier: f32,
}

#[derive(serde::Deserialize)]
pub struct GrabberConfig {
    /// Orbit and visuals radius
    pub radius: f32,
    /// Margin that still allows interaction
    pub entry_margin: f32,
    /// Max speed the rope can handle before breaking
    pub max_speed: f32,
}

#[derive(serde::Deserialize)]
pub struct SlingshotConfig {
    /// Interaction radius
    pub orbit_radius: f32,
    /// The maximum duration of the orbit
    pub orbit_duration: f32,
    /// Cooldown before next interaction
    pub orbit_cooldown: f32,
}
