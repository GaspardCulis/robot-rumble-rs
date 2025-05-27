use bevy::{platform::collections::HashMap, prelude::*};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct ProjectilesConfig(pub HashMap<Projectile, ProjectileConfig>);

#[derive(
    Component, Default, Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Deserialize,
)]
pub enum Projectile {
    #[default]
    Bullet,
    Rocket,
    Laser,
    ShockWave,
}

#[derive(serde::Deserialize)]
pub struct ProjectileConfig {
    pub stats: ProjectileStats,
    pub skin: ProjectileSkin,
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub struct ProjectileStats {
    pub mass: u32,
    pub radius: f32,
    pub damage: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectileSkin {
    /// Path to the projectile sprite image
    pub sprite: String,
    pub scale: f32,
}
