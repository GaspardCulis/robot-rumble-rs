use crate::{
    GameState,
    core::{
        audio::AudioSFX,
        physics::{PhysicsSet, Position, Rotation, Velocity},
    },
    entities::projectile::{
        Damage, DecayTimer, Projectile,
        config::{BH_BULLET_DECAY_TIME, ProjectilesAssets, ProjectilesConfig},
    },
};
use bevy::{math::ops::cos, prelude::*};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use bevy_kira_audio::{AudioChannel, AudioControl, AudioInstance, AudioTween, PlaybackState};
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

pub mod assets;
pub mod config;

use assets::WeaponsAssets;
use config::{WeaponStats, WeaponType, WeaponsConfig, WeaponsConfigAssets};

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
    pub cooldown_timer: Timer,
    pub reload_timer: Timer,
}

#[derive(Component, Reflect)]
#[relationship_target(relationship = super::Weapon)]
pub struct Owner(Entity);

#[derive(Component, Reflect)]
struct AudioReload(Handle<AudioInstance>);

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AudioReload>()
            .register_type::<WeaponType>()
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
                    (
                        add_stats_component,
                        add_sprite,
                        mode_change_detection,
                        visibility_change_detection,
                    )
                        .before(PhysicsSet::Player)
                        .run_if(in_state(GameState::InGame)),
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
    assets: Res<WeaponsConfigAssets>,
    configs: Res<Assets<WeaponsConfig>>,
) {
    let Some(config) = configs.get(&assets.config) else {
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
    config_assets: Res<WeaponsConfigAssets>,
) {
    let Some(config) = configs.get(&config_assets.config) else {
        warn!("Couldn't load WeaponsConfig");
        return;
    };

    for (weapon_entity, weapon_type) in query.iter() {
        if let Some(weapon_config) = config.0.get(weapon_type)
            && let Some(weapon_assets) = assets.get(weapon_type)
        {
            commands.entity(weapon_entity).insert((
                Sprite::from_image(weapon_assets.skin.clone()),
                Transform::from_xyz(0.0, 0.0, super::skin::PLAYER_SKIN_ZINDEX + 1.0)
                    .with_scale(Vec3::splat(weapon_config.skin.scale)),
            ));
        }
    }
}

// this will be very useful a bit further in visuals and sound effects
fn visibility_change_detection(
    query: Query<Option<&AudioReload>, (Changed<Visibility>, With<WeaponType>)>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for maybe_audio in query.iter() {
        if let Some(audio) = maybe_audio {
            let handle = &audio.0;
            if let Some(instance) = audio_instances.get_mut(handle) {
                match instance.state() {
                    PlaybackState::Paused { .. } => {
                        // There are a lot of control methods defined on the instance
                        instance.resume(AudioTween::default());
                    }
                    PlaybackState::Playing { .. } => {
                        instance.pause(AudioTween::default());
                    }
                    _ => {}
                }
            }
        }
    }
}

// do some effects on mode changes
fn mode_change_detection(
    mut commands: Commands,
    query: Query<(Entity, &WeaponMode, &WeaponType, Option<&AudioReload>), Changed<WeaponMode>>,
    assets: Res<WeaponsAssets>,
    sfx_channel: Res<AudioChannel<AudioSFX>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    for (entity, mode, weapon_type, maybe_audio) in query.iter() {
        let Some(weapon_assets) = assets.get(weapon_type) else {
            warn!("Couldn't load WeaponAssets");
            return;
        };
        match mode {
            WeaponMode::Reloading => {
                if let Some(sound) = weapon_assets.reload.clone() {
                    let handle: Handle<AudioInstance> = sfx_channel.play(sound).handle();
                    commands.entity(entity).insert(AudioReload(handle));
                } else {
                    warn!("Reload sound for current weapon is not implemented!");
                }
            }
            _ => {
                if let Some(audio) = maybe_audio {
                    let handle = &audio.0;
                    if let Some(instance) = audio_instances.get_mut(handle) {
                        instance.stop(AudioTween::default());
                    }

                    commands.entity(entity).remove::<AudioReload>();
                }
            }
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
    projectiles_assets: Res<ProjectilesAssets>,
    projectiles_configs: Res<Assets<ProjectilesConfig>>,
    time: Res<bevy_ggrs::RollbackFrameCount>,
    assets: Res<WeaponsAssets>,
    sfx_channel: Res<AudioChannel<AudioSFX>>,
) {
    let Some(projectiles_config) = projectiles_configs.get(&projectiles_assets.config) else {
        warn!("Couldn't load ProjectileConfig");
        return;
    };

    for (mut state, mut mode, position, velocity, rotation, stats, owner, weapon_type) in
        weapon_query.iter_mut()
    {
        if *mode == WeaponMode::Triggered && state.can_fire() {
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
            if let Some(weapon_assets) = assets.get(weapon_type) {
                sfx_channel.play(weapon_assets.fire.clone());
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

impl WeaponState {
    pub fn can_fire(&self) -> bool {
        self.cooldown_timer.finished() && self.current_ammo > 0
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
