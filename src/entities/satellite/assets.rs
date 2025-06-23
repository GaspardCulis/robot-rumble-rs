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
    pub orbit_radius: f32,
    pub min_angular_speed: f32,
    pub orbit_duration: f32,
    pub orbit_cooldown: f32,
    pub bump_radius: f32,
    pub bump_multiplier: f32,
    pub grabber_radius: f32,
}
