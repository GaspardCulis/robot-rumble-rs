use crate::{
    core::physics::{PhysicsSet, Position, Rotation, Velocity},
    entities::bullet::{BULLET_SPEED, Bullet},
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

mod config;

pub use config::{WeaponStats, WeaponType};

#[derive(Component, Clone, Default)]
pub struct Triggered(pub bool);

#[derive(Component, Clone, Debug)]
/// Weapon in-game state
pub struct WeaponState {
    current_ammo: usize,
    cooldown_timer: Timer,
    reload_timer: Timer,
}

#[derive(Resource)]
struct WeaponsConfigHandle(Handle<config::WeaponsConfig>);

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components_with::<WeaponType, Name>(|| Name::new("Weapon"))
            .register_required_components::<WeaponType, Triggered>()
            .add_plugins(RonAssetPlugin::<config::WeaponsConfig>::new(&[]))
            .add_systems(Startup, load_weapons_config)
            .add_systems(
                Update,
                (
                    #[cfg(debug_assertions)]
                    handle_config_reload,
                    add_stats_component.run_if(resource_exists::<WeaponsConfigHandle>),
                ),
            )
            .add_systems(
                GgrsSchedule,
                (tick_weapon_timers, fire_weapon_system)
                    .chain()
                    .in_set(PhysicsSet::Player)
                    .after(super::update_weapon),
            );
    }
}

fn load_weapons_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapons_config = WeaponsConfigHandle(asset_server.load("config/weapons.ron"));
    commands.insert_resource(weapons_config);
}

fn add_stats_component(
    mut commands: Commands,
    query: Query<(Entity, &WeaponType), (With<WeaponType>, Without<WeaponStats>)>,
    config_handle: Res<WeaponsConfigHandle>,
    config_assets: Res<Assets<config::WeaponsConfig>>,
) {
    let config = if let Some(c) = config_assets.get(config_handle.0.id()) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (weapon_entity, weapon_type) in query.iter() {
        if let Some(weapon_stats) = config.0.get(weapon_type) {
            // Overrides weapon state if present
            commands.entity(weapon_entity).insert(WeaponState {
                current_ammo: weapon_stats.magazine_size,
                cooldown_timer: Timer::new(weapon_stats.cooldown, TimerMode::Once),
                reload_timer: Timer::new(weapon_stats.reload_time, TimerMode::Once),
            });

            commands.entity(weapon_entity).insert(weapon_stats.clone());
        }
    }
}

/// Also handles ammo reloads for convenience
fn tick_weapon_timers(mut query: Query<(&mut WeaponState, &WeaponStats)>, time: Res<Time>) {
    for (mut state, stats) in query.iter_mut() {
        state.cooldown_timer.tick(time.delta());
        state.reload_timer.tick(time.delta());

        // Verify current_ammo is 0 to avoid a subtle bug where we fire when WeaponState is instanciated
        if state.reload_timer.just_finished() && state.current_ammo == 0 {
            state.current_ammo = stats.magazine_size;
        }
    }
}

fn fire_weapon_system(
    mut commands: Commands,
    mut weapon_query: Query<
        (
            &mut WeaponState,
            &Triggered,
            &Position,
            &Velocity,
            &Rotation,
            &WeaponStats,
        ),
        With<WeaponType>,
    >,
    time: Res<bevy_ggrs::RollbackFrameCount>,
) {
    for (mut state, triggered, position, velocity, rotation, stats) in weapon_query.iter_mut() {
        if triggered.0 && state.cooldown_timer.finished() && state.current_ammo > 0 {
            // Putting it here is important as query iter order is non-deterministic
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(time.0 as u64);
            for _ in 0..stats.shot_bullet_count {
                let random_angle = rng.random_range(-stats.spread..stats.spread);

                let bullet = (
                    Bullet,
                    Position(position.0),
                    Velocity(
                        Vec2::from_angle(rotation.0 + random_angle) * BULLET_SPEED + velocity.0,
                    ),
                );

                commands.spawn(bullet).add_rollback();
            }

            state.current_ammo -= 1;
            state.cooldown_timer.reset();

            if state.current_ammo == 0 {
                state.reload_timer.reset();
            }
        }
    }
}

#[cfg(debug_assertions)]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<config::WeaponsConfig>>,
    weapons: Query<Entity, With<WeaponStats>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id: _ } => {
                for weapon in weapons.iter() {
                    commands.entity(weapon).remove::<WeaponStats>();
                }
            }
            _ => {}
        };
    }
}
