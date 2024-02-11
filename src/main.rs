use core::CorePlugins;

use bevy::prelude::*;
use entities::EntitiesPlugins;

mod core;
mod entities;
mod utils;

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
        .run();
}
