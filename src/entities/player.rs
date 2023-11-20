use bevy::prelude::*;

use crate::core::{
    gravity::Mass,
    physics::{Position, Velocity},
};

const PLAYER_MASS: u32 = 800;
const PLAYER_VELOCITY: f32 = 500.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerInputVelocity(Vec2);

#[derive(Resource)]
pub struct PlayerSprite(Handle<Image>);

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    mass: Mass,
    sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            position: Position(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
            mass: Mass(PLAYER_MASS),
            sprite_bundle: SpriteBundle {
                ..Default::default()
            },
        }
    }
}

pub fn spawn_player(mut commands: Commands, sprite: Res<PlayerSprite>) {
    commands.spawn((
        SpriteBundle {
            texture: sprite.0.clone(),
            transform: Transform::from_scale(Vec3::splat(0.1)),
            ..default()
        },
        PlayerBundle {
            position: Position(Vec2 { x: 100., y: 0. }),
            velocity: Velocity(Vec2 { x: 0., y: 100. }),
            ..Default::default()
        },
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, handle_keys);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_sprite = asset_server.load("img/player.png");
    commands.insert_resource(PlayerSprite(player_sprite));
}

fn handle_keys(mut query: Query<&mut PlayerInputVelocity>, keyboard_input: Res<Input<KeyCode>>) {
    let mut velocity = query.single_mut();

    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::Z]) {
        todo!("Jump");
    }

    if keyboard_input.pressed(KeyCode::D) {
        velocity.0.x = PLAYER_VELOCITY;
    }
    if keyboard_input.pressed(KeyCode::Q) {
        velocity.0.x = -PLAYER_VELOCITY;
    }
}
