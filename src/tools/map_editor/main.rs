use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::ecs::error::{ErrorContext, GLOBAL_ERROR_HANDLER};
use bevy::prelude::*;

use bevy_inspector_egui::{bevy_egui::*, quick::WorldInspectorPlugin};
use robot_rumble::*;

mod controller;
mod interaction;
mod model;
mod utils;
mod view;

fn main() {
    // Avoid some crashes/hangups when EGui context is `get_mut` when window closes
    GLOBAL_ERROR_HANDLER
        .set(|error: BevyError, _ctx: ErrorContext| {
            warn!("Encountered error: {}", error);
        })
        .expect("The error handler can only be set once.");

    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            MeshPickingPlugin,
            DebugPickingPlugin,
        ))
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..Default::default()
        })
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(entities::EntitiesPlugins)
        .add_plugins(core::CorePlugins)
        .add_plugins(level::LevelPlugins)
        .add_plugins(network::NetworkPlugin)
        .insert_resource(robot_rumble::Args {
            players: 1,
            synctest: false,
            level_path: None,
        })
        .init_state::<GameState>()
        .add_plugins(controller::ControllerPlugin)
        .add_plugins(interaction::InteractionPlugin)
        .add_plugins(model::ModelPlugin)
        .add_plugins(view::ViewPlugin)
        .add_systems(
            PreUpdate,
            (|mut mode: ResMut<DebugPickingMode>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    DebugPickingMode::Normal => DebugPickingMode::Noisy,
                    DebugPickingMode::Noisy => DebugPickingMode::Disabled,
                }
            })
            .distributive_run_if(bevy::input::common_conditions::input_just_pressed(
                KeyCode::F3,
            )),
        )
        .add_systems(
            Update,
            (
                enable_camera_picking,
                disable_camera_follow,
                utils::update_planet_radius,
            ),
        )
        .run();
}

fn enable_camera_picking(mut commands: Commands, camera: Query<Entity, With<Camera2d>>) -> Result {
    let camera = camera.single()?;
    commands.entity(camera).insert(MeshPickingCamera);

    Ok(())
}

fn disable_camera_follow(
    mut commands: Commands,
    query: Query<Entity, With<core::camera::CameraFollowTarget>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<core::camera::CameraFollowTarget>();
    }
}
