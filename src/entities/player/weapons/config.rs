use std::time::Duration;

use bevy::prelude::*;

/// Availabale weapons to spawn
// TODO: Move to config file weapons properties
#[derive(Component, Default, Debug)]
pub enum WeaponType {
    #[default]
    Pistol,
    Shotgun,
    Riffle,
}

#[derive(Debug, Component)]
/// Static weapon properties
pub struct WeaponStats {
    pub cooldown: Duration,
    pub magazine_size: u32,
    pub reload_time: Duration,
    pub damage_multiplyer: f32,
}
