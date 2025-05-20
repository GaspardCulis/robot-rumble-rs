use crate::{
    core::physics::{Position, Rotation, Velocity},
    entities::bullet::{BULLET_SPEED, Bullet},
};
use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::AddRollbackCommandExtension;
use config::WeaponsConfig;

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
struct WeaponsConfigHandle(Handle<WeaponsConfig>);

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components::<WeaponType, Triggered>()
            .add_plugins(RonAssetPlugin::<WeaponsConfig>::new(&[]))
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
    weapon_query: Query<(&Triggered, &Position, &Velocity, &Rotation), With<WeaponType>>,
) {
    for (triggered, position, velocity, Rotation(direction)) in weapon_query.iter() {
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
