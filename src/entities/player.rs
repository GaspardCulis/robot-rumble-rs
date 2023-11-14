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
            transform: Transform::from_xyz(100., 0., 0.),
            ..default()
        },
        Player,
    ));
}
