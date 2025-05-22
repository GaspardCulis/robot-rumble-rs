use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsSchedule;

mod config;
pub use config::Projectile;
#[derive(Resource)]
struct ProjectilesConfigHandle(Handle<config::ProjectilesConfig>);

use super::planet::Planet;
use crate::core::{
    collision::{CollisionPlugin, CollisionShape, CollisionState},
    gravity::{Mass, Passive},
    physics::{PhysicsSet, Rotation, Velocity},
};

// Autodespawn timer
#[derive(Component)]
pub struct DecayTimer(pub Timer);

#[derive(Component, Reflect, Clone, Copy)]
pub struct Damage(pub f32);

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components::<Projectile, CollisionShape>()
            .register_required_components_with::<Projectile, Transform>(|| {
                Transform::from_scale(Vec3::splat(1.5))
            })
            .register_required_components_with::<Projectile, Rotation>(|| Rotation(0.))
            .register_required_components_with::<Projectile, Passive>(|| Passive)
            .register_required_components_with::<Projectile, Name>(|| Name::new("Projectile"))
            .add_plugins(RonAssetPlugin::<config::ProjectilesConfig>::new(&[]))
            .add_plugins(CollisionPlugin::<Projectile, Planet>::new())
            .add_systems(Startup, load_projectiles_config)
            .add_systems(
                Update,
                (
                    #[cfg(debug_assertions)]
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
                    add_physical_properties.before(PhysicsSet::Gravity),
                    check_collisions.after(PhysicsSet::Collision),
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
    query: Query<(Entity, &Projectile), (Without<Mass>, Without<Damage>)>,
    config_handle: Res<ProjectilesConfigHandle>,
    config_assets: Res<Assets<config::ProjectilesConfig>>,
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
                .insert(Mass(projectile_stats.mass))
                .insert(Damage(projectile_stats.damage));
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

fn tick_projectile_timer(
    mut commands: Commands,
    mut projectiles_querry: Query<(Entity, &mut DecayTimer), With<Projectile>>,
    time: Res<Time>,
) {
    for (projectile, mut despawn_timer) in projectiles_querry.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.just_finished() {
            commands.entity(projectile).despawn();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    query: Query<(Entity, &CollisionState<Projectile, Planet>), With<Projectile>>,
) {
    for (projectile, collision_state) in query.iter() {
        if collision_state.collider.is_some() {
            commands.entity(projectile).despawn_recursive();
        }
    }
}

#[cfg(debug_assertions)]
fn handle_config_reload(
    mut commands: Commands,
    mut events: EventReader<AssetEvent<config::ProjectilesConfig>>,
    projectiles: Query<Entity, With<Sprite>>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id: _ } => {
                for projectile in projectiles.iter() {
                    commands.entity(projectile).remove::<Sprite>();
                }
            }
            _ => {}
        };
    }
}
