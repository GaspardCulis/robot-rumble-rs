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

use super::planet::{Planet, Radius};

const PLAYER_MASS: u32 = 800;
const PLAYER_VELOCITY: f32 = 500.;
const PLAYER_RADIUS: f32 = 10.;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct PlayerInputVelocity(Vec2);

#[derive(AssetCollection, Resource)]
struct PlayerAssets {
    #[asset(path = "img/player.png")]
    player: Handle<Image>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum PlayerState {
    #[default]
    InAir,
    OnGround,
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
            .add_state::<PlayerState>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, handle_keys)
            .add_systems(Update, player_physics);
    }
}

fn spawn_player(mut commands: Commands, sprite: Res<PlayerAssets>) {
    commands.spawn((PlayerBundle {
        position: Position(Vec2 { x: 0., y: 500. }),
        velocity: Velocity(Vec2 { x: 0., y: 0. }),
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
    player_state: Res<State<PlayerState>>,
    time: Res<Time>,
) {
    let (mut input_velocity, mut velocity, rotation) = query.single_mut();
    let delta = time.delta_seconds();

    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Z])
        && *player_state.get() == PlayerState::OnGround
    {
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
    mut player_query: Query<
        (
            &mut Position,
            &mut Rotation,
            &mut Velocity,
            &PlayerInputVelocity,
        ),
        Without<Planet>,
    >,
    planet_query: Query<(&Position, &Radius), With<Planet>>,
    mut player_state: ResMut<NextState<PlayerState>>,
    time: Res<Time>,
) {
    let (mut player_position, mut player_rotation, mut velocity, input_velocity) =
        player_query.single_mut();

    // Find nearest planet (asserts that one planet exists)
    let mut nearest_position = Vec2::ZERO;
    let mut nearest_radius: u32 = 0;
    let mut nearest_distance = f32::MAX;
    for (position, radius) in planet_query.iter() {
        let distance = position.0.distance(player_position.0) - radius.0 as f32;
        if distance < nearest_distance {
            nearest_position = position.0;
            nearest_radius = radius.0;
            nearest_distance = distance;
        }
    }

    // Rotate towards it
    let target_angle = (nearest_position.y - player_position.0.y)
        .atan2(nearest_position.x - player_position.0.x)
        + PI / 2.;
    let mut short_angle = (target_angle - player_rotation.0) % (2. * PI);
    short_angle = (2. * short_angle) % (2. * PI) - short_angle;
    player_rotation.0 += short_angle * time.delta_seconds() * 1.;

    player_position.0 += input_velocity.0 * time.delta_seconds();

    // Check if collides
    if nearest_distance - PLAYER_RADIUS <= 0. {
        let collision_normal = (player_position.0 - nearest_position).normalize();
        velocity.0 = Vec2::ZERO;

        let clip_position =
            nearest_position + collision_normal * (PLAYER_RADIUS + nearest_radius as f32);
        player_position.0 = clip_position;

        player_state.set(PlayerState::OnGround);
    }
}
