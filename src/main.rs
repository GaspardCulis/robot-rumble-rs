use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use core::CorePlugins;
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
        .add_plugins(WorldInspectorPlugin::new().run_if(|| cfg!(debug_assertions)))
        .run();
}
