use std::time::Duration;

use bevy::{prelude::*, utils::HashMap};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WeaponsConfig(pub HashMap<WeaponType, WeaponStats>);

/// Availabale weapons to spawn
// TODO: Move to config file weapons properties
#[derive(Component, Default, Debug, Hash, PartialEq, Eq, serde::Deserialize)]
pub enum WeaponType {
    #[default]
    Pistol,
    Shotgun,
    Rifle,
}

#[serde_with::serde_as]
#[derive(Debug, Component, serde::Deserialize)]
/// Static weapon properties
pub struct WeaponStats {
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub cooldown: Duration,
    pub magazine_size: u32,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub reload_time: Duration,
    pub damage_multiplier: f32,
}
