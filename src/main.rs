use core::gravity::{GravityPlugin, Mass, Position, Velocity};

use bevy::prelude::*;

mod core;
mod entities;

fn add_players(mut commands: Commands) {
    commands.spawn((
        Position { x: 0., y: 0. },
        Velocity { x: 0., y: 0. },
        Mass(0),
    ));
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
        .add_plugins(GravityPlugin)
        .add_systems(Startup, add_players)
        .run();
}
