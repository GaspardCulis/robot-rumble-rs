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
        Position(Vec2 { x: 0., y: 0. }),
        Velocity(Vec2 { x: 20., y: 10. }),
    ));
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sprite_movement);
    }
}

fn sprite_movement(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;
    }
}
