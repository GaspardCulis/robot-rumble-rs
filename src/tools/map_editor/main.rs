use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
use bevy::ecs::error::{ErrorContext, GLOBAL_ERROR_HANDLER};
use bevy::prelude::*;

use bevy_inspector_egui::{bevy_egui::*, quick::WorldInspectorPlugin};
use rand::Rng;
use robot_rumble::{
    core::{
        physics::Position,
        worldgen::{WorldgenConfig, WorldgenConfigHandle},
    },
    entities::planet::*,
    *,
};

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
                setup.run_if(resource_added::<WorldgenConfigHandle>),
                utils::update_planet_radius,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut planet_events: EventWriter<SpawnPlanetEvent>,
    camera: Query<Entity, With<Camera2d>>,
    worldgen_config_handle: Res<WorldgenConfigHandle>,
    worldgen_configs: Res<Assets<WorldgenConfig>>,
) -> Result {
    let worldgen_config = worldgen_configs
        .get(&worldgen_config_handle.0)
        .ok_or(BevyError::from("Failed to get worlgen config"))?;

    planet_events.write(SpawnPlanetEvent {
        position: Position(Vec2::ZERO),
        radius: Radius(worldgen_config.central_star_radius),
        r#type: PlanetType::Star,
        seed: rand::rng().random(),
    });

    // Enable picking on camera
    let camera = camera.single()?;
    commands.entity(camera).insert(MeshPickingCamera);

    Ok(())
}
