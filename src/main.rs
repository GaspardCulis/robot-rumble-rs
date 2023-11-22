use core::{physics::Position, CorePlugins};

use bevy::prelude::*;
use entities::{player::Player, EntitiesPlugins};

mod core;
mod entities;
mod utils;

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
                        resolution: (1280.0, 720.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(CorePlugins)
        .add_plugins(EntitiesPlugins)
        .add_systems(Update, log_player_pos)
        .run();
}
