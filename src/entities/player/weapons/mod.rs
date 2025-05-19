use crate::{
    core::physics::{Position, Rotation, Velocity},
    entities::bullet::{BULLET_SPEED, Bullet},
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::AddRollbackCommandExtension;
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

mod config;

pub use config::{WeaponStats, WeaponType};

#[derive(Component, Default)]
pub struct Triggered(pub bool);

#[derive(Debug, Component)]
/// Weapon in-game state
struct WeaponState {
    current_ammo: u32,
    reload_timer: Timer,
    last_shot_ts: Option<f32>,
}

#[derive(Resource)]
struct WeaponsConfigHandle(Handle<config::WeaponsConfig>);

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components::<WeaponType, Triggered>()
            .add_plugins(RonAssetPlugin::<config::WeaponsConfig>::new(&[]))
            .add_systems(Startup, load_weapons_config)
            .add_systems(
                Update,
                (
                    #[cfg(debug_assertions)]
                    handle_config_reload,
                    add_stats_component.run_if(resource_exists::<WeaponsConfigHandle>),
                    fire_weapon_system,
                ),
            );
    }
}

fn load_weapons_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let weapons_config = WeaponsConfigHandle(asset_server.load("config/weapons.ron"));
    commands.insert_resource(weapons_config);
}

fn add_stats_component(
    mut commands: Commands,
    query: Query<(Entity, &WeaponType, Has<WeaponState>), (With<WeaponType>, Without<WeaponStats>)>,
    config_handle: Res<WeaponsConfigHandle>,
    config_assets: Res<Assets<config::WeaponsConfig>>,
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
    weapon_query: Query<
        (&Triggered, &Position, &Velocity, &Rotation, &WeaponStats),
        With<WeaponType>,
    >,
    time: Res<bevy_ggrs::RollbackFrameCount>,
) {
    for (triggered, position, velocity, rotation, stats) in weapon_query.iter() {
        if triggered.0 {
            // Putting it here is important as query iter order is non-deterministic
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(time.0 as u64);
            let random_angle = rng.random_range(-stats.spread..stats.spread);

            let bullet = (
                Bullet,
                Position(position.0),
                Velocity(Vec2::from_angle(rotation.0 + random_angle) * BULLET_SPEED + velocity.0),
            );

            commands.spawn(bullet).add_rollback();
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
