use crate::{
    core::{
        audio::SoundEvent,
        physics::{PhysicsSet, Position, Rotation, Velocity},
    },
    entities::projectile::{
        Damage, DecayTimer, Projectile,
        config::{BH_BULLET_DECAY_TIME, ProjectilesAssets, ProjectilesConfig},
    },
};
use bevy::{math::ops::cos, prelude::*};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use config::{WeaponStats, WeaponType, WeaponsAssets, WeaponsConfig};
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

pub mod config;

#[derive(Component, Clone, PartialEq, Default, Reflect)]
pub enum WeaponMode {
    #[default]
    Idle,
    Triggered,
    Reloading,
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct WeaponState {
    pub current_ammo: usize,
    cooldown_timer: Timer,
    reload_timer: Timer,
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = super::Weapon)]
pub struct Owner(Entity);

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WeaponType>()
            .register_type::<WeaponStats>()
            .register_type::<WeaponState>()
            .register_type::<WeaponMode>()
            .register_required_components_with::<WeaponType, Name>(|| Name::new("Weapon"))
            .register_required_components::<WeaponType, WeaponMode>()
            .add_systems(
                Update,
                (
                    #[cfg(feature = "dev_tools")]
                    handle_config_reload,
                    (add_stats_component, add_sprite)
                        .before(PhysicsSet::Player)
                        .run_if(resource_exists::<WeaponsAssets>),
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

fn add_stats_component(
    mut commands: Commands,
    query: Query<(Entity, &WeaponType), Without<WeaponStats>>,
    assets: Res<WeaponsAssets>,
    configs: Res<Assets<WeaponsConfig>>,
) {
    let config = if let Some(c) = configs.get(&assets.config) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (weapon_entity, weapon_type) in query.iter() {
        if let Some(weapon_config) = config.0.get(weapon_type) {
            let weapon_stats = &weapon_config.stats;

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

fn add_sprite(
    mut commands: Commands,
    query: Query<(Entity, &WeaponType), Without<Sprite>>,
    assets: Res<WeaponsAssets>,
    configs: Res<Assets<WeaponsConfig>>,
    asset_server: Res<AssetServer>,
) {
    let config = if let Some(c) = configs.get(&assets.config) {
        c
    } else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (weapon_entity, weapon_type) in query.iter() {
        if let Some(weapon_config) = config.0.get(weapon_type) {
            let skin = weapon_config.skin.clone();

            commands.entity(weapon_entity).insert((
                Sprite::from_image(asset_server.load(skin.sprite)),
                Transform::from_xyz(0.0, 0.0, super::skin::PLAYER_SKIN_ZINDEX + 1.0)
                    .with_scale(Vec3::splat(skin.scale)),
            ));
        }
    }
}

/// Also handles ammo reloads for convenience
fn tick_weapon_timers(
    mut query: Query<(&mut WeaponState, &WeaponStats, &mut WeaponMode), With<Position>>,
    time: Res<Time>,
) {
    for (mut state, stats, mut mode) in query.iter_mut() {
        state.cooldown_timer.tick(time.delta());
        if *mode == WeaponMode::Reloading {
            state.reload_timer.tick(time.delta());
        }
        // Verify current_ammo is 0 to avoid a subtle bug where we fire when WeaponState is instantiated
        if state.reload_timer.finished() && state.current_ammo < stats.magazine_size {
            state.current_ammo = stats.magazine_size;
            *mode = WeaponMode::Idle;
        }
    }
}

fn fire_weapon_system(
    mut commands: Commands,
    mut weapon_query: Query<(
        &mut WeaponState,
        &mut WeaponMode,
        &Position,
        &Velocity,
        &Rotation,
        &WeaponStats,
        &Owner,
        &WeaponType,
    )>,
    mut owner_query: Query<&mut Velocity, Without<WeaponType>>,
    mut events: EventWriter<SoundEvent>,
    projectiles_assets: Res<ProjectilesAssets>,
    projectiles_configs: Res<Assets<ProjectilesConfig>>,
    time: Res<bevy_ggrs::RollbackFrameCount>,
    assets: Res<WeaponsAssets>,
    weapon_configs: Res<Assets<WeaponsConfig>>,
    asset_server: Res<AssetServer>,
) {
    let Some(projectiles_config) = projectiles_configs.get(&projectiles_assets.config) else {
        warn!("Couldn't load ProjectileConfig");
        return;
    };

    for (mut state, mut mode, position, velocity, rotation, stats, owner, weapon_type) in
        weapon_query.iter_mut()
    {
        if (*mode == WeaponMode::Triggered)
            && state.cooldown_timer.finished()
            && state.current_ammo > 0
        {
            // Putting it here is important as query iter order is non-deterministic
            let mut rng = Xoshiro256PlusPlus::seed_from_u64(time.0 as u64);
            for _ in 0..stats.shot_bullet_count {
                if let Some(projectile_config) = projectiles_config.0.get(&stats.projectile) {
                    let projectile_stats = &projectile_config.stats;
                    let random_angle = rng.random_range(-stats.spread..stats.spread);

                    let projectile_direction = Vec2::from_angle(rotation.0 + random_angle);
                    // 1 if player and bullet directions points to the same direction, 0 if perpendicular, -1 if opposite
                    let player_velocity_multiplier = cos(velocity.0.angle_to(projectile_direction));
                    let added_velocity = velocity.0.length() * player_velocity_multiplier;

                    let new_projectile = (
                        stats.projectile,
                        // Avoid bullet hitting player firing
                        Position(position.0 + Vec2::from_angle(rotation.0) * super::PLAYER_RADIUS),
                        Velocity(projectile_direction * (stats.projectile_speed + added_velocity)),
                        Damage(stats.damage_multiplier * projectile_stats.damage),
                    );
                    let mut projectile_entity = commands.spawn(new_projectile);
                    // Add decay for black hole bullets
                    if stats.projectile == Projectile::Blackhole {
                        projectile_entity.insert(DecayTimer(Timer::from_seconds(
                            BH_BULLET_DECAY_TIME,
                            TimerMode::Once,
                        )));
                    }
                    projectile_entity.add_rollback();
                } else {
                    warn!("Empy projectile config!")
                }
            }
            // make sound
            // shitcode, pls gsprd mk hndls
            let config = if let Some(c) = weapon_configs.get(&assets.config) {
                c
            } else {
                warn!("Couldn't load WeaponsConfig");
                return;
            };
            if let Some(weapon_config) = config.0.get(weapon_type) {
                let sound: Handle<AudioSource> =
                    asset_server.load(weapon_config.sounds.fire.clone());
                events.write(SoundEvent { handle: sound });
            }

            state.current_ammo -= 1;
            // Reset timers if shooting
            state.cooldown_timer.reset();
            state.reload_timer.reset();
            // Auto-reload if empty mag
            if state.current_ammo == 0 {
                *mode = WeaponMode::Reloading;
            }
            // Recoil
            if let Ok(mut owner_velocity) = owner_query.get_mut(owner.0) {
                owner_velocity.0 -= Vec2::from_angle(rotation.0) * stats.recoil;
            }
        }
    }
}

#[cfg(feature = "dev_tools")]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<config::WeaponsConfig>>,
    weapons: Query<Entity, (With<WeaponStats>, With<Sprite>)>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id: _ } = event {
            for weapon in weapons.iter() {
                commands.entity(weapon).remove::<WeaponStats>();
                commands.entity(weapon).remove::<Sprite>();
            }
        };
    }
}
