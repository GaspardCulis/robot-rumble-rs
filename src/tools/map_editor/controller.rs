use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    prelude::*,
};
use bevy_inspector_egui::bevy_egui::input::egui_wants_any_pointer_input;
use rand::Rng as _;
use robot_rumble::{core::physics, entities::planet};

use crate::model::UiState;

pub struct ControllerPlugin;
impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_planet_pointer_observer,
                update_planet_radius,
                handle_spawn_planet_button,
                (drag_camera_view, handle_right_click).run_if(not(egui_wants_any_pointer_input)),
            ),
        );
    }
}

fn handle_right_click(
    mut ui_state: ResMut<UiState>,
    window: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) -> Result {
    let window = window.single()?;

    if mouse_button.just_released(MouseButton::Right) {
        if ui_state.context_menu_position.is_none() {
            ui_state.context_menu_position =
                Some(window.cursor_position().unwrap_or(window.size() / 2.));
        } else {
            ui_state.context_menu_position = None;
        }
    }

    if mouse_button.just_released(MouseButton::Left) {
        // Discard context menu
        ui_state.context_menu_position = None;
    }

    Ok(())
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
        } else {
            // You dumb fuck
        }
    }

    Ok(())
}

fn add_planet_pointer_observer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &planet::Radius), Added<planet::Planet>>,
) {
    for (entity, radius) in query.iter() {
        commands
            .entity(entity)
            .insert((
                Mesh2d(meshes.add(Mesh::from(Circle::new(radius.0 as f32)))),
                Pickable::default(),
            ))
            .observe(
                move |mut trigger: Trigger<Pointer<Click>>, mut ui_state: ResMut<UiState>| {
                    let click_event: &Pointer<Click> = trigger.event();
                    if click_event.button == PointerButton::Primary {
                        ui_state.focused_planet = Some(entity);
                    }
                    trigger.propagate(false);
                },
            );
    }
}
//
// Also handles zoom cuz im lazy
fn drag_camera_view(
    mut query: Query<&mut Transform, With<Camera2d>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut mouse_wheel_event_reader: EventReader<MouseWheel>,
) {
    for mut camera_transform in query.iter_mut() {
        if mouse_button.pressed(MouseButton::Left) {
            let delta = mouse_motion.delta * camera_transform.scale.xy();

            camera_transform.translation.x -= delta.x;
            camera_transform.translation.y += delta.y;
        }

        for ev in mouse_wheel_event_reader.read() {
            let current_scale = camera_transform.scale.x;
            camera_transform.scale = Vec3::ONE * (current_scale + ev.y * 0.2).clamp(0.1, 10.0);
        }
    }
}

fn update_planet_radius(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<
        (&planet::Radius, &Mesh2d, &Children),
        (With<planet::Planet>, Changed<planet::Radius>),
    >,
) {
    for (radius, mesh, children) in query.iter() {
        let mesh = meshes.get_mut(mesh).unwrap();
        *mesh = Mesh::from(Circle::new(radius.0 as f32));

        for material_layer in children {
            commands.entity(*material_layer).despawn();
        }
    }
}

fn mouse_pos_to_world(mouse_pos: &Vec2, camera_transform: &Transform, window_size: &Vec2) -> Vec2 {
    let abs_mouse_pos = mouse_pos - window_size / 2.0;

    camera_transform
        .transform_point(Vec3::new(abs_mouse_pos.x, -abs_mouse_pos.y, 0.0))
        .xy()
}
