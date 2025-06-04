use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsSchedule;
pub mod config;
pub use config::{Projectile, ProjectilesConfig};
#[derive(Resource)]
pub struct ProjectilesConfigHandle(pub Handle<config::ProjectilesConfig>);

use super::{planet::Planet, player::Player};
use crate::core::{
    collision::{CollisionPlugin, CollisionShape, CollisionState},
    gravity::{Mass, Passive},
    physics::{PhysicsSet, Position, Rotation, Velocity},
};

type PlanetCollision = CollisionState<Projectile, Planet>;
type PlayerCollision = CollisionState<Projectile, Player>;

/// Autodespawn timer. Yields `ProjectileDecayedEvent`.
#[derive(Component, Debug, Clone, Reflect)]
pub struct DecayTimer(pub Timer);

// Possible fix for events
#[derive(Event, Debug, Reflect, Clone)]
pub struct ProjectileDecayedEvent {
    /// Where the projectile decayed
    pub position: Position,
    /// Which type of projectile it is.
    /// Can be None if the entity isn't a projectile, so that decay functionnality can be used in other modules.
    /// Might get moved somewhere else in the future
    pub r#type: Option<Projectile>,
}

#[derive(Component, Reflect, Clone, Copy)]
pub struct Damage(pub f32);

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<DecayTimer>()
            .register_required_components::<Projectile, CollisionShape>()
            .register_required_components_with::<Projectile, Transform>(|| {
                Transform::from_scale(Vec3::splat(1.5))
            })
            .register_required_components_with::<Projectile, Rotation>(|| Rotation(0.))
            .register_required_components_with::<Projectile, Passive>(|| Passive)
            .register_required_components_with::<Projectile, Name>(|| Name::new("Projectile"))
            .add_event::<ProjectileDecayedEvent>()
            .add_plugins(RonAssetPlugin::<config::ProjectilesConfig>::new(&[]))
            .add_plugins(CollisionPlugin::<Projectile, Planet>::new())
            .add_plugins(CollisionPlugin::<Projectile, Player>::new())
            .add_systems(Startup, load_projectiles_config)
            .add_systems(
                Update,
                (
                    #[cfg(feature = "dev_tools")]
                    handle_config_reload,
                    add_sprite,
                    rotate_sprite,
                )
                    .chain()
                    .run_if(resource_exists::<ProjectilesConfigHandle>),
            )
            .add_systems(
                GgrsSchedule,
                (
                    tick_decay_timers.before(PhysicsSet::Player),
                    add_physical_properties
                        .before(PhysicsSet::Gravity)
                        .after(PhysicsSet::Player),
                    check_player_collisions
                        .before(PhysicsSet::Movement)
                        .after(PhysicsSet::Interaction),
                    check_planet_collisions.after(PhysicsSet::Collision),
                ),
            );
    }
}

fn load_projectiles_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let projectiles_config = ProjectilesConfigHandle(asset_server.load("config/projectiles.ron"));
    commands.insert_resource(projectiles_config);
}

fn add_physical_properties(
    mut commands: Commands,
    query: Query<(Entity, &Projectile), Without<Mass>>,
    config_handle: Res<ProjectilesConfigHandle>,
    config_assets: Res<Assets<ProjectilesConfig>>,
) {
    let config = if let Some(c) = config_assets.get(config_handle.0.id()) {
        c
    } else {
        warn!("Couldn't load ProjectileConfig");
        return;
    };

    for (projectile_entity, projectile_type) in query.iter() {
        if let Some(projectile_config) = config.0.get(projectile_type) {
            let projectile_stats = &projectile_config.stats;

            commands
                .entity(projectile_entity)
                .insert(Mass(projectile_stats.mass));
        }
    }
}

fn add_sprite(
    mut commands: Commands,
    query: Query<(Entity, &Projectile), Without<Sprite>>,
    config_handle: Res<ProjectilesConfigHandle>,
    config_assets: Res<Assets<config::ProjectilesConfig>>,
    asset_server: Res<AssetServer>,
) {
    let config = if let Some(c) = config_assets.get(config_handle.0.id()) {
        c
    } else {
        warn!("Couldn't load ProjectilesConfig");
        return;
    };

    for (projectile_entity, projectile_type) in query.iter() {
        if let Some(projectile_config) = config.0.get(projectile_type) {
            let skin = projectile_config.skin.clone();

            commands.entity(projectile_entity).insert((
                Sprite::from_image(asset_server.load(skin.sprite)),
                Transform::from_xyz(0.0, 0.0, 10.0 + 1.0).with_scale(Vec3::splat(skin.scale)),
            ));
        }
    }
}

fn rotate_sprite(mut query: Query<(&mut Rotation, &Velocity), (With<Projectile>, With<Sprite>)>) {
    for (mut rotation, velocity) in query.iter_mut() {
        rotation.0 = -velocity.angle_to(Vec2::X);
    }
}

fn tick_decay_timers(
    mut commands: Commands,
    mut projectile_decay_events: EventWriter<ProjectileDecayedEvent>,
    mut projectiles_query: Query<(Entity, Option<&Projectile>, &Position, &mut DecayTimer)>,
    time: Res<Time>,
) {
    for (entity, projectile, position, mut despawn_timer) in projectiles_query.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.just_finished() {
            // TODO: Control on events rollback
            projectile_decay_events.write(ProjectileDecayedEvent {
                position: position.clone(),
                r#type: projectile.cloned(),
            });
            commands.entity(entity).despawn();
        }
    }
}

fn check_planet_collisions(
    mut commands: Commands,
    query: Query<(Entity, &PlanetCollision), With<Projectile>>,
) {
    for (projectile, planet_collision) in query.iter() {
        if planet_collision.collides {
            commands.entity(projectile).despawn();
        }
    }
}

fn check_player_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Velocity, &Mass, &PlayerCollision), With<Projectile>>,
    mut player_query: Query<(&mut Velocity, &Mass), Without<Projectile>>,
) {
    for (projectile, projectile_velocity, projectile_mass, player_collision) in query.iter() {
        if player_collision.collides {
            if let Some(closest_player) = player_collision.closest
                && let Ok((mut player_velocity, player_mass)) = player_query.get_mut(closest_player)
            {
                let knockback_force = projectile_velocity.0 * projectile_mass.0 as f32;
                player_velocity.0 += knockback_force / player_mass.0 as f32;
            }

            commands.entity(projectile).despawn();
        }
    }
}

#[cfg(feature = "dev_tools")]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<config::ProjectilesConfig>>,
    projectiles: Query<Entity, With<Sprite>>,
) {
    for event in events.read() {
        if let AssetEvent::Modified { id: _ } = event {
            for projectile in projectiles.iter() {
                commands.entity(projectile).remove::<Sprite>();
                commands.entity(projectile).remove::<Mass>();
                commands.entity(projectile).remove::<Damage>();
            }
        };
    }
}
