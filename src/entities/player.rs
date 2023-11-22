use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    core::{
        gravity::Mass,
        physics::{Position, Rotation, Velocity},
    },
    utils::math,
};

use super::planet::Radius;

const PLAYER_MASS: u32 = 800;
const PLAYER_VELOCITY: f32 = 500.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerInputVelocity(Vec2);

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "img/player.png")]
    player: Handle<Image>,
}

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    rotation: Rotation,
    input_velocity: PlayerInputVelocity,
    mass: Mass,
    sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            position: Position(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
            rotation: Rotation(0.),
            input_velocity: PlayerInputVelocity(Vec2::ZERO),
            mass: Mass(PLAYER_MASS),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<PlayerAssets>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, handle_keys)
            .add_systems(Update, player_physics)
            .add_systems(Update, rotate_to_nearest_planet);
    }
}

fn spawn_player(mut commands: Commands, sprite: Res<PlayerAssets>) {
    commands.spawn((PlayerBundle {
        position: Position(Vec2 { x: 100., y: 0. }),
        velocity: Velocity(Vec2 { x: 0., y: 100. }),
        rotation: Rotation(PI / 2.),
        sprite_bundle: SpriteBundle {
            texture: sprite.player.clone(),
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        },
        ..Default::default()
    },));
}

fn handle_keys(
    mut query: Query<(&mut PlayerInputVelocity, &mut Velocity, &Rotation)>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut input_velocity, mut velocity, rotation) = query.single_mut();
    let delta = time.delta_seconds();

    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Z]) {
        velocity.0 += Vec2::from_angle(rotation.0).rotate(Vec2::Y) * PLAYER_VELOCITY;
    }

    if keyboard_input.pressed(KeyCode::D) {
        input_velocity.0.x = math::lerp(input_velocity.0.x, PLAYER_VELOCITY, delta);
    }
    if keyboard_input.pressed(KeyCode::Q) {
        input_velocity.0.x = math::lerp(input_velocity.0.x, -PLAYER_VELOCITY, delta);
    }
    if keyboard_input.pressed(KeyCode::S) {
        input_velocity.0.y = math::lerp(input_velocity.0.y, -PLAYER_VELOCITY * 0.75, delta);
    }
}

fn player_physics(
    mut query: Query<(&mut Position, &mut Rotation, &PlayerInputVelocity)>,
    time: Res<Time>,
) {
    for (mut position, mut rotation, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds();
        rotation.0 += time.delta_seconds();
    }
}

fn rotate_to_nearest_planet(
    mut player_query: Query<(&mut Rotation, &Position), With<Player>>,
    planet_query: Query<(&Position, &Radius)>,
    time: Res<Time>,
) {
    let (mut player_rotation, player_position) = player_query.single_mut();

    // Find nearest planet
    let mut nearest_position = Vec2::ZERO;
    let mut nearest_distance = f32::MAX;
    for (position, radius) in planet_query.iter() {
        let distance = position.0.distance(player_position.0);
        if distance - (radius.0 as f32) < nearest_distance {
            nearest_position = position.0;
            nearest_distance = distance;
        }
    }

    let target_angle = (nearest_position.y - player_position.0.y)
        .atan2(nearest_position.x - player_position.0.x)
        + PI / 2.;
    let mut short_angle = (target_angle - player_rotation.0) % (2. * PI);
    short_angle = (2. * short_angle) % (2. * PI) - short_angle;

    player_rotation.0 += short_angle * time.delta_seconds() * 1.;
}
