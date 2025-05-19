use crate::{
    core::{
        gravity::Passive,
        physics::{Position, Rotation, Velocity},
    },
    entities::bullet::{BULLET_SPEED, Bullet},
};
use bevy::prelude::*;
use bevy_ggrs::AddRollbackCommandExtension;
use std::time::Duration;

/// Availabale weapons to spawn
// TODO: Move to config file weapons properties
#[derive(Component, Default, Debug)]
pub enum Weapon {
    #[default]
    Pistol,
    Shotgun,
    Riffle,
}

struct CurrentAmmo(u32);

#[derive(Component)]
pub struct Triggered(pub bool);

#[derive(Debug, Component)]
/// Static weapon properties
struct WeaponStats {
    cooldown: Duration,
    magazine_size: u32,
    reload_time: Duration,
    damage_multiplyer: f32,
}

#[derive(Debug, Component)]
/// Weapon in-game state
struct WeaponState {
    current_ammo: u32,
    reload_timer: Timer,
    last_shot_ts: Option<f32>,
}
#[derive(Debug, Component)]
pub struct Direction(pub Vec2);

#[derive(Bundle)]
pub struct WeaponBundle {
    marker: Weapon,
    position: Position,
    velocity: Velocity,
    _rotation: Rotation,
    direction: Direction,
    is_triggered: Triggered,
    stats: WeaponStats,
    state: WeaponState,
    passive: Passive,
}

impl WeaponBundle {
    pub fn new(weapon_type: Weapon, position: Position) -> Self {
        // TODO: define config file for uniform build
        match weapon_type {
            Weapon::Pistol => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(300),
                    magazine_size: 15,
                    reload_time: Duration::from_millis(1000),
                    damage_multiplyer: 1.0,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    marker: weapon_type,
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
            Weapon::Shotgun => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(500),
                    magazine_size: 8,
                    reload_time: Duration::from_millis(5000),
                    damage_multiplyer: 1.0,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    marker: weapon_type,
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
            Weapon::Riffle => {
                let stats = WeaponStats {
                    cooldown: Duration::from_millis(300),
                    magazine_size: 10,
                    reload_time: Duration::from_millis(2500),
                    damage_multiplyer: 1.1,
                };

                let state = WeaponState {
                    current_ammo: stats.magazine_size,
                    reload_timer: Timer::default(),
                    last_shot_ts: None,
                };

                Self {
                    marker: weapon_type,
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

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_weapon_system);
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
    weapon_query: Query<(&Triggered, &Position, &Velocity, &Direction), With<Weapon>>,
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
