use crate::{
    GameState,
    core::physics::{PhysicsSet, Position, Rotation, Velocity},
    entities::projectile::{
        Damage, DecayTimer, Projectile,
        config::{BH_BULLET_DECAY_TIME, ProjectilesAssets, ProjectilesConfig},
    },
    level::limit,
};
use bevy::{math::ops::cos, prelude::*};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsSchedule};
use rand::{Rng as _, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;

pub mod assets;
pub mod config;
mod sfx;

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

#[derive(Event)]
pub enum WeaponEvent {
    Fire(Entity),
    ReloadStart(Entity),
    ReloadEnd(Entity),
    Equipped(Entity),
    UnEquipped(Entity),
}

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Owner>()
            .register_type::<WeaponType>()
            .register_type::<WeaponStats>()
            .register_type::<WeaponState>()
            .register_type::<WeaponMode>()
            .register_required_components_with::<WeaponType, Name>(|| Name::new("Weapon"))
            .register_required_components::<WeaponType, WeaponMode>()
            .add_event::<WeaponEvent>()
            .add_plugins(sfx::WeaponSFXPlugin)
            .add_systems(
                Update,
                (
                    #[cfg(feature = "dev_tools")]
                    handle_config_reload,
                    (
                        add_stats,
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
                (init_state, tick_weapon_timers, fire_weapon_system)
                    .chain()
                    .in_set(PhysicsSet::Collision)
                    .after(limit::handle_player_death),
            );
    }
}

fn add_stats(
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
            commands.entity(weapon_entity).insert(weapon_stats.clone());
        } else {
            warn!("Couldn't get config stats for {weapon_type:?}");
        }
    }
}

fn init_state(mut commands: Commands, query: Query<(Entity, &WeaponStats), Without<WeaponState>>) {
    for (weapon_entity, weapon_stats) in query.iter() {
        commands.entity(weapon_entity).insert(WeaponState {
            current_ammo: weapon_stats.magazine_size,
            cooldown_timer: Timer::new(weapon_stats.cooldown, TimerMode::Once),
            reload_timer: Timer::new(weapon_stats.reload_time, TimerMode::Once),
        });
        commands.entity(weapon_entity).insert(WeaponMode::Idle);
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
    query: Query<(Entity, &Visibility), (Changed<Visibility>, With<WeaponType>)>,
    mut events: EventWriter<WeaponEvent>,
) {
    for (entity, visibility) in query.iter() {
        match visibility {
            Visibility::Inherited => {}
            Visibility::Hidden => {
                events.write(WeaponEvent::UnEquipped(entity));
            }
            Visibility::Visible => {
                events.write(WeaponEvent::Equipped(entity));
            }
        };
    }
}

fn mode_change_detection(
    query: Query<(Entity, &WeaponMode), Changed<WeaponMode>>,
    mut events: EventWriter<WeaponEvent>,
) {
    for (entity, mode) in query.iter() {
        match mode {
            WeaponMode::Idle => {}
            WeaponMode::Triggered => {}
            WeaponMode::Reloading => {
                events.write(WeaponEvent::ReloadStart(entity));
            }
        };
    }
}

/// Also handles ammo reloads for convenience
fn tick_weapon_timers(
    mut query: Query<(Entity, &mut WeaponState, &mut WeaponMode, &WeaponStats), With<Position>>,
    mut events: EventWriter<WeaponEvent>,
    time: Res<Time>,
) {
    for (entity, mut state, mut mode, stats) in query.iter_mut() {
        state.cooldown_timer.tick(time.delta());
        if *mode == WeaponMode::Reloading {
            state.reload_timer.tick(time.delta());
        }

        if state.reload_timer.finished() && state.current_ammo < stats.magazine_size {
            state.current_ammo = stats.magazine_size;
            state.reload_timer.reset();
            *mode = WeaponMode::Idle;

            events.write(WeaponEvent::ReloadEnd(entity));
        }
    }
}

fn fire_weapon_system(
    mut commands: Commands,
    mut weapon_query: Query<(
        Entity,
        &mut WeaponState,
        &mut WeaponMode,
        &Position,
        &Velocity,
        &Rotation,
        &WeaponStats,
        &Owner,
    )>,
    mut owner_query: Query<&mut Velocity, Without<Owner>>,
    mut events: EventWriter<WeaponEvent>,
    projectiles_assets: Res<ProjectilesAssets>,
    projectiles_configs: Res<Assets<ProjectilesConfig>>,
    time: Res<bevy_ggrs::RollbackFrameCount>,
) {
    let Some(projectiles_config) = projectiles_configs.get(&projectiles_assets.config) else {
        warn!("Couldn't load ProjectileConfig");
        return;
    };

    for (entity, mut state, mut mode, position, velocity, rotation, stats, owner) in
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
                    warn!("Empty projectile config!");
                }
            }

            events.write(WeaponEvent::Fire(entity));

            state.current_ammo -= 1;
            // Reset timers if shooting
            state.cooldown_timer.reset();
            state.reload_timer.reset();
            // Auto-reload if empty mag
            if state.current_ammo == 0 {
                *mode = WeaponMode::Reloading;

                events.write(WeaponEvent::ReloadStart(entity));
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

impl WeaponEvent {
    pub fn get_entity(&self) -> Entity {
        *match self {
            WeaponEvent::Fire(entity) => entity,
            WeaponEvent::ReloadStart(entity) => entity,
            WeaponEvent::ReloadEnd(entity) => entity,
            WeaponEvent::Equipped(entity) => entity,
            WeaponEvent::UnEquipped(entity) => entity,
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
