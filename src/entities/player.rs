use bevy::prelude::*;

use crate::core::{
    gravity::Mass,
    physics::{Position, Velocity},
};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    marker: Player,
    position: Position,
    velocity: Velocity,
    mass: Mass,
    sprite: SpriteSheetBundle,
}

pub fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/player.png"),
            transform: Transform::from_scale(Vec3::splat(0.1)),

            ..default()
        },
        Player,
        Position(Vec2 { x: 100., y: 0. }),
        Velocity(Vec2 { x: 0., y: 100. }),
        Mass(800),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {}
}
