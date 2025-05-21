use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::{
    GameState,
    core::{
        gravity::{Mass, Passive},
        physics::{PhysicsSet, Position, Rotation, Velocity},
    },
};

use super::planet::Radius;

pub const BULLET_SPEED: f32 = 1200.;
pub const BULLET_MASS: u32 = 20;
pub const BULLET_DESPAWN_TIME: Duration = Duration::from_millis(500);

// Autodespawn timer
#[derive(Component)]
struct DespawnTimer(Timer);
#[derive(Component, Reflect, Clone, Copy)]
#[require(Visibility)]
pub struct Bullet;

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.register_required_components_with::<Bullet, Transform>(|| {
            Transform::from_scale(Vec3::splat(1.5))
        })
        .register_required_components_with::<Bullet, Rotation>(|| Rotation(0.0))
        .register_required_components_with::<Bullet, Mass>(|| Mass(BULLET_MASS))
        .register_required_components_with::<Bullet, Passive>(|| Passive)
        .register_required_components_with::<Bullet, DespawnTimer>(|| {
            DespawnTimer(Timer::new(BULLET_DESPAWN_TIME, TimerMode::Once))
        })
        .register_required_components_with::<Bullet, Name>(|| Name::new("Bullet"))
        .add_systems(
            Update,
            (add_sprite, rotate_sprite).chain().before(check_collisions),
        )
        .add_systems(Update, tick_bullet_timer.before(check_collisions))
        .add_systems(
            GgrsSchedule,
            check_collisions
                .run_if(in_state(GameState::InGame))
                .after(PhysicsSet::Movement),
        );
    }
}

fn add_sprite(
    mut commands: Commands,
    query: Query<Entity, (Added<Bullet>, Without<Sprite>)>,
    asset_server: Res<AssetServer>,
) {
    for bullet in query.iter() {
        commands
            .entity(bullet)
            .insert(Sprite::from_image(asset_server.load("bullet.png")));
    }
}

fn rotate_sprite(mut query: Query<(&mut Rotation, &Velocity), (With<Bullet>, With<Sprite>)>) {
    for (mut rotation, velocity) in query.iter_mut() {
        rotation.0 = -velocity.angle_to(Vec2::X);
    }
}

fn tick_bullet_timer(
    mut commands: Commands,
    mut bulltets_querry: Query<(Entity, &mut DespawnTimer), With<Bullet>>,
    time: Res<Time>,
) {
    for (bullet, mut despawn_timer) in bulltets_querry.iter_mut() {
        despawn_timer.0.tick(time.delta());
        if despawn_timer.0.just_finished() {
            commands.entity(bullet).despawn();
        }
    }
}

fn check_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Position), With<Bullet>>,
    planet_query: Query<(&Position, &Radius)>,
) {
    for (bullet, bullet_position) in bullet_query.iter() {
        for (planet_position, planet_radius) in planet_query.iter() {
            let distance = bullet_position.distance(planet_position.0) - planet_radius.0 as f32;
            if distance <= 0.0 {
                commands.entity(bullet).despawn_recursive();
            }
        }
    }
}
