use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    core::{
        gravity::Mass,
        physics::{Position, Velocity},
    },
    utils::math,
};

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
            input_velocity: PlayerInputVelocity(Vec2::ZERO),
            mass: Mass(PLAYER_MASS),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}

fn spawn_player(mut commands: Commands, sprite: Res<PlayerAssets>) {
    commands.spawn((PlayerBundle {
        position: Position(Vec2 { x: 100., y: 0. }),
        velocity: Velocity(Vec2 { x: 0., y: 100. }),
        sprite_bundle: SpriteBundle {
            texture: sprite.player.clone(),
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        },
        ..Default::default()
    },));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<PlayerAssets>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, handle_keys)
            .add_systems(Update, player_physics);
    }
}

fn handle_keys(
    mut query: Query<&mut PlayerInputVelocity>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut velocity = query.single_mut();
    let delta = time.delta_seconds();

    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Z]) {
        todo!("Jump");
    }

    if keyboard_input.pressed(KeyCode::D) {
        velocity.0.x = math::lerp(velocity.0.x, PLAYER_VELOCITY, delta);
    }
    if keyboard_input.pressed(KeyCode::Q) {
        velocity.0.x = math::lerp(velocity.0.x, -PLAYER_VELOCITY, delta);
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.0.y = math::lerp(velocity.0.y, -PLAYER_VELOCITY * 0.75, delta);
    }
}

fn player_physics(mut query: Query<(&mut Position, &PlayerInputVelocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds();
    }
}
