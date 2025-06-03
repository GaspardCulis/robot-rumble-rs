use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::input::EguiWantsInput;
use robot_rumble::{
    core::physics,
    entities::{planet, satellite},
};

use crate::model::UiState;

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                add_planet_pointer_observers,
                add_satellite_pointer_observers,
                add_window_pointer_observers,
            ),
        );
    }
}

fn add_window_pointer_observers(mut commands: Commands, query: Query<Entity, Added<Window>>) {
    for window in query.iter() {
        commands
            .entity(window)
            .observe(handle_window_click)
            .observe(handle_window_drag)
            .observe(handle_window_scroll);
    }
}

fn handle_window_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    egui_wants_input_resource: Res<EguiWantsInput>,
) -> Result {
    if !egui_wants_input_resource.wants_any_pointer_input() {
        let event = trigger.event();
        if event.button == PointerButton::Primary {
            let mut camera = query.single_mut()?;

            let scaled_delta = event.delta * camera.scale.xy();

            camera.translation.x -= scaled_delta.x;
            camera.translation.y += scaled_delta.y;

            trigger.propagate(false);
        }
    }

    Ok(())
}

fn handle_window_scroll(
    mut trigger: Trigger<Pointer<Scroll>>,
    mut query: Query<&mut Transform, With<Camera2d>>,
    egui_wants_input_resource: Res<EguiWantsInput>,
) -> Result {
    if !egui_wants_input_resource.wants_any_pointer_input() {
        let event = trigger.event();
        let mut camera = query.single_mut()?;

        let current_scale = camera.scale.x;
        camera.scale = Vec3::ONE * (current_scale - event.y * 0.2).clamp(0.1, 10.0);

        trigger.propagate(false);
    }

    Ok(())
}

fn handle_window_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut ui_state: ResMut<UiState>,
    query: Query<&Window>,
    egui_wants_input_resource: Res<EguiWantsInput>,
) -> Result {
    if !egui_wants_input_resource.wants_any_pointer_input() {
        let event = trigger.event();
        let window = query.single()?;
        if event.button == PointerButton::Secondary {
            ui_state.context_menu_position =
                Some(window.cursor_position().unwrap_or(window.size() / 2.));
        } else if event.button == PointerButton::Primary {
            ui_state.context_menu_position = None;
        }

        trigger.propagate(false);
    }

    Ok(())
}

fn add_planet_pointer_observers(
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
            .observe(handle_click)
            .observe(handle_drag);
    }
}

/// Sets clicked entity as `focused_entity`
fn handle_click(
    mut trigger: Trigger<Pointer<Click>>,
    mut ui_state: ResMut<UiState>,
    egui_wants_input_resource: Res<EguiWantsInput>,
) {
    if !egui_wants_input_resource.wants_any_pointer_input() {
        let event: &Pointer<Click> = trigger.event();
        if event.button == PointerButton::Primary {
            ui_state.focused_entity = Some(event.target);
        }
        trigger.propagate(false);
    }
}

fn handle_drag(
    mut trigger: Trigger<Pointer<Drag>>,
    mut query: Query<&mut physics::Position>,
    camera: Query<&Transform, With<Camera2d>>,
    egui_wants_input_resource: Res<EguiWantsInput>,
) -> Result {
    if !egui_wants_input_resource.wants_any_pointer_input() {
        let event = trigger.event();
        if event.button == PointerButton::Primary {
            let camera = camera.single()?;
            let mut position = query.get_mut(event.target)?;

            position.0 += event.delta * Vec2::new(1.0, -1.0) * camera.scale.xy();

            trigger.propagate(false);
        }
    }

    Ok(())
}

fn add_satellite_pointer_observers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<Entity, Added<satellite::Satellite>>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert((
                Mesh2d(meshes.add(Mesh::from(Circle::new(100.0)))),
                Pickable::default(),
            ))
            .observe(handle_click)
            .observe(handle_drag);
    }
}
