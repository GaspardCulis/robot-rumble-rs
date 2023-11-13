use core::{
    gravity::GravityPlugin,
    physics::{PhysicsPlugin, Position, Velocity},
    spritesheet::AnimatedSpritePlugin,
};

use bevy::prelude::*;
use entities::{planet::spawn_planet, player::Player};

mod core;
mod entities;

fn add_players(mut commands: Commands) {
    commands.spawn((
        Player,
        Position(Vec2 { x: 0., y: 0. }),
        Velocity(Vec2 { x: 2., y: 1. }),
    ));

    commands.spawn(Camera2dBundle::default());
}

fn log_player_pos(query: Query<&Position, With<Player>>) {
    let position = query.single().0;
    println!("Player position: {}", position);
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Game".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(PhysicsPlugin)
        .add_plugins(GravityPlugin)
        .add_plugins(AnimatedSpritePlugin)
        .add_systems(Startup, add_players)
        .add_systems(Startup, spawn_planet)
        .add_systems(Update, log_player_pos)
        .run();
}
