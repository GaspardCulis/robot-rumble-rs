use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsSchedule;
pub mod config;
pub use config::{Projectile, ProjectilesConfig};
#[derive(Resource)]
pub struct ProjectilesConfigHandle(pub Handle<config::ProjectilesConfig>);

use crate::core::{
    gravity::{Mass, Passive},
    physics::{PhysicsSet, Position, Rotation, Velocity},
};

use super::{
    planet::Radius,
    player::{self, PLAYER_RADIUS, Player},
};

// Autodespawn timer
#[derive(Component)]
pub struct DecayTimer(pub Timer);

#[derive(Component, Reflect, Clone, Copy)]
pub struct Damage(pub f32);
#[derive(Component, Reflect, Clone, Copy)]
pub struct Knockback(pub f32);

pub const KNOCKBACK_INFLUENCE: f32 = 0.75;

#[derive(Event)]
struct CollisionEvent {
    entity: Entity,
    position: Vec2,
    radius: f32,
    damage: f32,
    knockback: f32,
}
pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components_with::<Projectile, Transform>(|| {
            Transform::from_scale(Vec3::splat(1.5))
        })
        .register_required_components_with::<Projectile, Rotation>(|| Rotation(0.))
        .register_required_components_with::<Projectile, Passive>(|| Passive)
        .register_required_components_with::<Projectile, Name>(|| Name::new("Projectile"))
        .add_plugins(RonAssetPlugin::<config::ProjectilesConfig>::new(&[]))
        .add_event::<CollisionEvent>()
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
                check_collisions.after(PhysicsSet::Movement),
                take_hit,
            )
                .chain(),
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
    projectile_query: Query<(Entity, &Position, &Radius, &Knockback, &Damage)>,
    target_query: Query<(&Position, &Radius), Without<Projectile>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (
        projectile,
        projectile_position,
        projectile_radius,
        projectile_knockback,
        projectile_damage,
    ) in projectile_query.iter()
    {
        for (target_position, target_radius) in target_query.iter() {
            let distance = projectile_position.distance(target_position.0);
            let collision_distance = (projectile_radius.0 + target_radius.0) as f32;
            if distance <= collision_distance {
                collision_events.send(CollisionEvent {
                    entity: projectile,
                    position: projectile_position.0,
                    radius: projectile_radius.0 as f32,
                    damage: projectile_damage.0,
                    knockback: projectile_knockback.0,
                });
                commands.entity(projectile).despawn();
                break;
            }
        }
    }
}

fn take_hit(
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(&Position, &mut Velocity), With<Player>>,
) {
    for event in collision_events.read() {
        for (player_position, mut player_velocity) in player_query.iter_mut() {
            let distance = (event.position).distance(player_position.0);
            if distance <= (event.radius + PLAYER_RADIUS) {
                // apply just knockback for now
                let knockback_direction = (player_position.0 - event.position).normalize();
                let player_speed = player_velocity.length();
                let player_direction;
                if player_speed > 0.0 {
                    player_direction = player_velocity.normalize();
                } else {
                    player_direction = Vec2::ZERO;
                };
                let distance_factor = (1.0 - distance / event.radius).clamp(0.0, 1.0);
                let new_direction = ((1.0 - distance_factor) * player_direction
                    + distance_factor * knockback_direction)
                    .normalize_or_zero();
                let new_direction = new_direction.normalize_or_zero(); // to avoid NaN if zero vector
                player_velocity.0 = new_direction * player_speed;
            }
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
                    commands.entity(projectile).remove::<Mass>();
                    commands.entity(projectile).remove::<Damage>();
                    commands.entity(projectile).remove::<Knockback>();
                }
            }
            _ => {}
        };
    }
}
