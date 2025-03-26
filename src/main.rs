use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use clap::Parser;

mod core;
mod entities;
mod network;
mod utils;

#[derive(Parser, Resource, Debug)]
pub struct Args {
    /// Runs the game in synctest mode
    #[clap(long)]
    pub synctest: bool,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    MatchMaking,
    WorldGen,
    InGame,
}

fn main() {
    let args = Args::parse();
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Game".into(),
                    resolution: (1280.0, 720.0).into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            })
            .build(),
    )
    .add_plugins(core::CorePlugins)
    .add_plugins(entities::EntitiesPlugins)
    .add_plugins(network::NetworkPlugin)
    .init_state::<GameState>()
    .insert_resource(args);

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
    }

    app.run();
}
