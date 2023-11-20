use bevy::prelude::*;

use crate::core::{
    gravity::Mass,
    physics::{Position, Velocity},
};

const PLAYER_MASS: u32 = 800;

#[derive(Component)]
pub struct Player;

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
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let player_sprite = asset_server.load("img/player.png");
    commands.insert_resource(PlayerSprite(player_sprite));
}
