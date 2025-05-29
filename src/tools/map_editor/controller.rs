use bevy::prelude::*;
use rand::Rng as _;
use robot_rumble::{core::physics, entities::planet};

use crate::{model::UiState, utils::mouse_pos_to_world};

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_spawn_planet_button);
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
