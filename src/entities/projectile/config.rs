use bevy::{prelude::*, utils::HashMap};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct ProjectilesConfig(pub HashMap<Projectile, ProjectileConfig>);

// TODO: add a config file
pub static BLACKHOLE_RADIUS: u32 = 60;
pub static BH_BULLET_DECAY_TIME: f32 = 0.5;
pub const BLAST_RADIUS: f32 = 20.;

#[derive(
    Component, Default, Reflect, Clone, Copy, Debug, Hash, PartialEq, Eq, serde::Deserialize,
)]
pub enum Projectile {
    #[default]
    Bullet,
    Blackhole,
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
    pub knockback: f32,
    pub damage: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectileSkin {
    /// Path to the projectile sprite image
    pub sprite: String,
    pub scale: f32,
}
