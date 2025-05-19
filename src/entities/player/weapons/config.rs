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
#[derive(Debug, Component, Clone, serde::Deserialize)]
/// Static weapon properties
pub struct WeaponStats {
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub cooldown: Duration,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub reload_time: Duration,
    pub magazine_size: usize,
    pub damage_multiplier: f32,
    /// Amount of spawned bullets per-shot. Still counts as one ammo
    pub shot_bullet_count: usize,
    pub recoil: f32,
    pub spread: f32,
}
