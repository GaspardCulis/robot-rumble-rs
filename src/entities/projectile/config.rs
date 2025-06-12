use bevy::{platform::collections::HashMap, prelude::*};
use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: add a config file
pub static BLACKHOLE_RADIUS: u32 = 60;
pub static BH_BULLET_DECAY_TIME: f32 = 0.5;

#[derive(AssetCollection, Resource)]
pub struct ProjectilesAssets {
    #[asset(path = "config/config.projectiles.ron")]
    pub config: Handle<ProjectilesConfig>,
}

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct ProjectilesConfig(pub HashMap<Projectile, ProjectileConfig>);

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
    #[allow(dead_code)] // Will be used later when we have different projectiles
    pub radius: f32,
    pub damage: f32,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct ProjectileSkin {
    /// Path to the projectile sprite image
    pub sprite: String,
    pub scale: f32,
}
