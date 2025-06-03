use bevy::prelude::*;
use rand::Rng as _;
use robot_rumble::{
    core::{physics, worldgen},
    entities::{planet, satellite},
    level::save,
};

use crate::{model::UiState, utils::mouse_pos_to_world};

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

        satellite_events.write(satellite::SpawnSatelliteEvent {
            position: physics::Position(spawn_position),
            kind: ui_state.satellite_kind_input,
            // FIX: Config file or constant, will be done when we have better satellite sprites
            scale: 0.7,
        });

        ui_state.context_menu_position = None;
        ui_state.buttons.spawn_satellite = false;
    }

    Ok(())
}

fn handle_save_map_button(
    ui_state: Res<UiState>,
    planets_query: Query<
        (
            &physics::Position,
            &planet::Radius,
            &planet::PlanetType,
            &worldgen::GenerationSeed,
        ),
        With<planet::Planet>,
    >,
    satellites_query: Query<
        (
            &physics::Position,
            Has<satellite::bumper::Bumper>,
            Has<satellite::grabber::Grabber>,
            Has<satellite::graviton::Graviton>,
        ),
        With<satellite::Satellite>,
    >,
) -> Result {
    if ui_state.buttons.save_map {
        let planets = planets_query
            .iter()
            .map(|(position, radius, r#type, seed)| save::PlanetSave {
                position: position.0,
                radius: radius.0,
                r#type: *r#type,
                seed: seed.0,
            })
            .collect();

        let satellites = satellites_query
            .iter()
            .map(|(position, bumper, grabber, graviton)| {
                let kind = if bumper {
                    satellite::SatelliteKind::Bumper
                } else if grabber {
                    satellite::SatelliteKind::Grabber
                } else if graviton {
                    satellite::SatelliteKind::Graviton
                } else {
                    unimplemented!("Wtf man ?!")
                };

                save::SatelliteSave {
                    position: position.0,
                    kind,
                }
            })
            .collect();

        let save_file = save::LevelSave {
            planets,
            satellites,
        };

        save_file.save(&ui_state.save_file_path)?;
    }

    Ok(())
}

fn handle_load_map_button(
    mut commands: Commands,
    mut load_level_save_events: EventWriter<save::LoadLevelSaveEvent>,
    mut ui_state: ResMut<UiState>,
    entities: Query<Entity, Or<(With<planet::Planet>, With<satellite::Satellite>)>>,
) -> Result {
    if ui_state.buttons.load_map {
        // Clear out old map first
        entities
            .iter()
            .for_each(|entity| commands.entity(entity).despawn());
        ui_state.focused_entity = None;

        // Spawn saved entities
        load_level_save_events.write(save::LoadLevelSaveEvent {
            path: ui_state.save_file_path.clone().into(),
        });
    }

    Ok(())
}
