use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct WeaponsConfig(pub HashMap<WeaponType, WeaponConfig>);

/// Availabale weapons to spawn
// TODO: Move to config file weapons properties
#[derive(Component, Clone, Default, Debug, Hash, PartialEq, Eq, Reflect, serde::Deserialize)]
pub enum WeaponType {
    #[default]
    Pistol,
    Shotgun,
    Rifle,
    Sniper,
    Revolver,
    Pulse,
}

#[derive(serde::Deserialize)]
pub struct WeaponConfig {
    pub stats: WeaponStats,
    pub skin: WeaponSkin,
}

#[serde_with::serde_as]
#[derive(Debug, Component, Clone, Reflect, serde::Deserialize)]
/// Static weapon properties
pub struct WeaponStats {
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub cooldown: Duration,
    #[serde_as(as = "serde_with::DurationSecondsWithFrac")]
    pub reload_time: Duration,
    pub magazine_size: usize,
    pub damage_multiplier: f32,
    pub projectile_speed: f32,
    /// Amount of spawned bullets per-shot. Still counts as one ammo
    pub shot_bullet_count: usize,
    pub recoil: f32,
    pub spread: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct WeaponSkin {
    /// Path to the weapon sprite image
    pub sprite: String,
    pub scale: f32,
}
