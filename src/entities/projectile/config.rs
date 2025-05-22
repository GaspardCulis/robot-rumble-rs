use bevy::{prelude::*, utils::HashMap};

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct ProjectilesConfig(pub HashMap<ProjectileType, ProjectileConfig>);

#[derive(Component, Default, Debug, Hash, PartialEq, Eq, serde::Deserialize)]
pub enum ProjectileType {
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
    pub damage: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectileSkin {
    /// Path to the projectile sprite image
    pub sprite: String,
    pub scale: f32,
}
