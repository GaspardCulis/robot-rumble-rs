use bevy::prelude::*;

use crate::core::{
    gravity::Mass,
    physics::{Position, Velocity},
};

const PLAYER_MASS: u32 = 800;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    mass: Mass,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            marker: Player,
            position: Position(Vec2::ZERO),
            velocity: Velocity(Vec2::ZERO),
            mass: Mass(PLAYER_MASS),
        }
    }
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/player.png"),
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
    fn build(&self, app: &mut App) {}
}
