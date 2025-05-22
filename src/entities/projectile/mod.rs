use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ggrs::GgrsSchedule;

mod config;
pub use config::ProjectileType;
#[derive(Resource)]
struct ProjectilesConfigHandle(Handle<config::ProjectilesConfig>);

use crate::{
    GameState,
    core::{
        gravity::{Mass, Passive},
        physics::{PhysicsSet, Position, Rotation, Velocity},
    },
};

use super::planet::Radius;

// Autodespawn timer
#[derive(Component)]
pub struct DecayTimer(pub Timer);
#[derive(Component, Reflect, Clone, Copy)]
#[require(Visibility)]
pub struct Projectile;

#[derive(Component, Reflect, Clone, Copy)]
pub struct Damage(pub f32);

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
        .add_systems(Startup, load_projectiles_config)
        .add_systems(
            Update,
            (
                #[cfg(debug_assertions)]
                handle_config_reload,
                (add_physical_properties, add_sprite)
                    .run_if(resource_exists::<ProjectilesConfigHandle>),
            ),
        )
        .add_systems(
            Update,
            (add_sprite, rotate_sprite).chain().before(check_collisions),
        )
        .add_systems(Update, tick_projectile_timer.before(check_collisions))
        .add_systems(
            GgrsSchedule,
            check_collisions
                .run_if(in_state(GameState::InGame))
                .after(PhysicsSet::Movement),
        );
    }
}

fn load_projectiles_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let projectiles_config = ProjectilesConfigHandle(asset_server.load("config/projectiles.ron"));
    commands.insert_resource(projectiles_config);
}

fn add_physical_properties(
    mut commands: Commands,
    query: Query<(Entity, &ProjectileType), (Without<Mass>, Without<Damage>)>,
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
    query: Query<(Entity, &ProjectileType), Without<Sprite>>,
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
    projectile_query: Query<(Entity, &Position), With<Projectile>>,
    planet_query: Query<(&Position, &Radius)>,
) {
    for (projectile, projectile_position) in projectile_query.iter() {
        for (planet_position, planet_radius) in planet_query.iter() {
            let distance = projectile_position.distance(planet_position.0) - planet_radius.0 as f32;
            if distance <= 0.0 {
                commands.entity(projectile).despawn_recursive();
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
                }
            }
            _ => {}
        };
    }
}
