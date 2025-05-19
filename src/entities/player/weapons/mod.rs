use crate::{
    core::{
        gravity::Passive,
        physics::{Position, Rotation, Velocity},
    },
    entities::bullet::{BULLET_SPEED, Bullet},
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::AddRollbackCommandExtension;
use config::WeaponsConfig;
use std::time::Duration;

mod config;

pub use config::{WeaponStats, WeaponType};

struct CurrentAmmo(u32);

#[derive(Component)]
pub struct Triggered(pub bool);

#[derive(Debug, Component)]
/// Weapon in-game state
struct WeaponState {
    current_ammo: u32,
    reload_timer: Timer,
    last_shot_ts: Option<f32>,
}
#[derive(Debug, Component)]
pub struct Direction(pub Vec2);

#[derive(Resource)]
struct WeaponsConfigHandle(Handle<WeaponsConfig>);

#[derive(Bundle)]
pub struct WeaponBundle {
    r#type: WeaponType,
    position: Position,
    velocity: Velocity,
    _rotation: Rotation,
    direction: Direction,
    is_triggered: Triggered,
    stats: WeaponStats,
    state: WeaponState,
    passive: Passive,
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<WeaponsConfig>::new(&[]))
            .add_systems(Startup, load_weapons_config)
            .add_systems(
                Update,
                (
                    add_stats_component.run_if(resource_exists::<WeaponsConfigHandle>),
                    fire_weapon_system,
                ),
            );
    }
}

impl WeaponBundle {
    pub fn new(weapon_type: WeaponType, position: Position) -> Self {
        // TODO: define config file for uniform build
        match weapon_type {
            WeaponType::Pistol => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(300),
                    magazine_size: 15,
                    reload_time: Duration::from_millis(1000),
                    damage_multiplier: 1.0,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    r#type: weapon_type,
                    position,
                    direction: Direction(Vec2::ZERO),
                    velocity: Velocity(Vec2::ZERO),
                    _rotation: Rotation(0.),
                    is_triggered: Triggered(false),
                    stats,
                    state,
                    passive: Passive,
                }
            }
            WeaponType::Shotgun => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(500),
                    magazine_size: 8,
                    reload_time: Duration::from_millis(5000),
                    damage_multiplier: 1.0,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    r#type: weapon_type,
                    position,
                    direction: Direction(Vec2::ZERO),
                    _rotation: Rotation(0.),
                    velocity: Velocity(Vec2::ZERO),
                    is_triggered: Triggered(false),
                    stats,
                    state,
                    passive: Passive,
                }
            }
            WeaponType::Rifle => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(300),
                    magazine_size: 10,
                    reload_time: Duration::from_millis(2500),
                    damage_multiplier: 1.1,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    r#type: weapon_type,
                    position,
                    direction: Direction(Vec2::ZERO),
                    _rotation: Rotation(0.),
                    velocity: Velocity(Vec2::ZERO),
                    is_triggered: Triggered(false),
                    stats,
                    state,
                    passive: Passive,
                }
            }
        }
    }
}

fn load_weapons_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapons_config = WeaponsConfigHandle(asset_server.load("config/weapons.ron"));
    commands.insert_resource(weapons_config);
}

fn add_stats_component(
    mut commands: Commands,
    query: Query<
        (Entity, &WeaponType, Has<WeaponState>),
        (Added<WeaponType>, Without<WeaponStats>),
    >,
    config_handle: Res<WeaponsConfigHandle>,
    config_assets: Res<Assets<WeaponsConfig>>,
) {
    let config = if let Some(c) = config_assets.get(config_handle.0.id()) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (weapon_entity, weapon_type, has_weapon_state) in query.iter() {
        if let Some(weapon_stats) = config.0.get(weapon_type) {
            if !has_weapon_state {
                commands.entity(weapon_entity).insert(WeaponState {
                    current_ammo: weapon_stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                });
            }

            commands.entity(weapon_entity).insert(weapon_stats.clone());
        }
    }
}

/*
/// update timer between shots
fn update_weapon_timer(mut query: Query<&mut Weapon>, time: Res<Time>) {
    for mut weapon in query.iter_mut() {
        weapon.last_shot.tick(time.delta());
        if weapon.last_shot.finished() {
            // Weapon can fire again
        }
    }
}
*/

fn fire_weapon_system(
    mut commands: Commands,
    weapon_query: Query<(&Triggered, &Position, &Velocity, &Direction), With<WeaponType>>,
) {
    for (triggered, position, velocity, Direction(direction)) in weapon_query.iter() {
        if triggered.0 {
            let bullet = (
                Bullet,
                Position(position.0),
                Velocity(direction * BULLET_SPEED + velocity.0),
            );

            commands.spawn(bullet).add_rollback();
        }
    }
}
