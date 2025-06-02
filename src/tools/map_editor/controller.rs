use bevy::prelude::*;
use rand::Rng as _;
use robot_rumble::{
    core::{physics, worldgen},
    entities::{planet, satellite},
};

use crate::{model::UiState, savefile, utils::mouse_pos_to_world};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_spawn_planet_button,
                handle_spawn_satellite_button,
                handle_save_map_button,
                handle_load_map_button,
            ),
        );
    }
}

fn handle_spawn_planet_button(
    mut ui_state: ResMut<UiState>,
    camera: Query<&Transform, With<Camera2d>>,
    window: Query<&Window>,
    mut planet_events: EventWriter<planet::SpawnPlanetEvent>,
) -> Result {
    let camera_transform = camera.single()?;
    let window = window.single()?;

    if ui_state.buttons.spawn_planet
        && let Some(click_position) = ui_state.context_menu_position
    {
        let spawn_position = mouse_pos_to_world(&click_position, camera_transform, &window.size());

        if let Ok(radius) = ui_state.radius_input.parse::<u32>() {
            planet_events.write(planet::SpawnPlanetEvent {
                position: physics::Position(spawn_position),
                radius: planet::Radius(radius),
                r#type: planet::PlanetType::Planet,
                seed: rand::rng().random(),
            });

            ui_state.context_menu_position = None;
            ui_state.buttons.spawn_planet = false;
        } else {
            // You dumb fuck
        }
    }

    Ok(())
}

fn handle_spawn_satellite_button(
    mut ui_state: ResMut<UiState>,
    camera: Query<&Transform, With<Camera2d>>,
    window: Query<&Window>,
    mut satellite_events: EventWriter<satellite::SpawnSatelliteEvent>,
) -> Result {
    let camera_transform = camera.single()?;
    let window = window.single()?;

    if ui_state.buttons.spawn_satellite
        && let Some(click_position) = ui_state.context_menu_position
    {
        let spawn_position = mouse_pos_to_world(&click_position, camera_transform, &window.size());

        if let Ok(scale) = ui_state.scale_input.parse::<f32>() {
            satellite_events.write(satellite::SpawnSatelliteEvent {
                position: physics::Position(spawn_position),
                kind: ui_state.satellite_kind_input,
                scale,
            });

            ui_state.context_menu_position = None;
            ui_state.buttons.spawn_satellite = false;
        } else {
            // You dumb fuck
        }
    }

    Ok(())
}

fn handle_save_map_button(
    ui_state: Res<UiState>,
    map_query: Query<
        (
            &physics::Position,
            &planet::Radius,
            &planet::PlanetType,
            &worldgen::GenerationSeed,
        ),
        With<planet::Planet>,
    >,
) -> Result {
    if ui_state.buttons.save_map {
        let planets = map_query
            .iter()
            .map(|(position, radius, r#type, seed)| savefile::PlanetSave {
                position: position.0,
                radius: radius.0,
                r#type: *r#type,
                seed: seed.0,
            })
            .collect();

        let save_file = savefile::SaveFile { planets };

        save_file.save(&ui_state.save_file_path)?;
    }

    Ok(())
}

fn handle_load_map_button(
    mut commands: Commands,
    mut planet_spawn_events: EventWriter<planet::SpawnPlanetEvent>,
    mut ui_state: ResMut<UiState>,
    entities: Query<Entity, With<planet::Planet>>,
) -> Result {
    if ui_state.buttons.load_map {
        let save = savefile::SaveFile::load(&ui_state.save_file_path)?;

        // Clear out old map first
        entities
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
        ui_state.focused_planet = None;

        // Spawn saved entities
        for savefile::PlanetSave {
            position,
            radius,
            r#type,
            seed,
        } in save.planets
        {
            planet_spawn_events.write(planet::SpawnPlanetEvent {
                position: physics::Position(position),
                radius: planet::Radius(radius),
                r#type,
                seed,
            });
        }
    }

    Ok(())
}
