use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

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
