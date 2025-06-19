#[cfg(feature = "dev_tools")]
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::{log, prelude::*};
#[cfg(feature = "embedded_assets")]
use bevy_embedded_assets::EmbeddedAssetPlugin;
#[cfg(feature = "dev_tools")]
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use clap::Parser as _;

use robot_rumble::*;

fn main() {
    let args = Args::parse();
    let mut app = App::new();

    #[cfg(feature = "embedded_assets")]
    app.add_plugins(EmbeddedAssetPlugin {
        mode: bevy_embedded_assets::PluginMode::ReplaceDefault,
    });

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
            .set(log::LogPlugin {
                filter: format!("{},discord_presence=off", log::DEFAULT_FILTER),
                ..default()
            })
            .build(),
    )
    .add_plugins(assets::AssetsPlugin)
    .add_plugins(core::CorePlugins)
    .add_plugins(entities::EntitiesPlugins)
    .add_plugins(level::LevelPlugins)
    .add_plugins(misc::MiscPlugins)
    .add_plugins(network::NetworkPlugin)
    .add_plugins(ui::UiPlugins)
    .init_state::<GameState>()
    .insert_resource(args);

    #[cfg(feature = "dev_tools")]
    app.add_plugins(EguiPlugin {
        enable_multipass_for_primary_context: true,
    })
    .add_plugins(WorldInspectorPlugin::new())
    .add_plugins(FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 16.0,
                ..default()
            },
            ..default()
        },
    });

    app.run();
}
