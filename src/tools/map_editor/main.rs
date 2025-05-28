use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    prelude::*,
};

use bevy_inspector_egui::{
    bevy_egui::{input::egui_wants_any_pointer_input, *},
    quick::WorldInspectorPlugin,
};
use rand::Rng;
use robot_rumble::{
    core::{
        physics::Position,
        worldgen::{WorldgenConfig, WorldgenConfigHandle},
    },
    entities::planet::*,
    *,
};

#[derive(Default, Resource)]
struct UiState {
    context_menu_position: Option<Vec2>,
    radius_input: String,
    buttons: ButtonsState,
}

#[derive(Default)]
struct ButtonsState {
    spawn_planet: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(entities::EntitiesPlugins)
        .add_plugins(core::CorePlugins)
        .init_state::<GameState>()
        .init_resource::<UiState>()
        .add_systems(
            Update,
            (
                setup.run_if(resource_added::<WorldgenConfigHandle>),
                (drag_camera_view, handle_right_click).run_if(not(egui_wants_any_pointer_input)),
            ),
        )
        .add_systems(
            EguiContextPass,
            (render_ui, handle_spawn_planet_button).chain(),
        )
        .run();
}

fn setup(
    mut planet_events: EventWriter<SpawnPlanetEvent>,
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

    Ok(())
}

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

fn render_ui(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) -> Result {
    let ctx = contexts
        .try_ctx_mut()
        .ok_or(BevyError::from("Couldn't get egui context"))?;

    if let Some(position) = ui_state.context_menu_position {
        egui::Window::new("Spawn planet")
            .collapsible(false)
            .fixed_pos((position.x, position.y))
            .fixed_size([200.0, 300.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let radius_label = ui.label("Radius: ");
                    ui.text_edit_singleline(&mut ui_state.radius_input)
                        .labelled_by(radius_label.id);
                });

                ui.separator();

                ui_state.buttons.spawn_planet = ui.button("Spawn planet").clicked();
            });
    }

    Ok(())
}

fn handle_spawn_planet_button(
    mut ui_state: ResMut<UiState>,
    camera: Query<&Transform, With<Camera2d>>,
    window: Query<&Window>,
    mut planet_events: EventWriter<SpawnPlanetEvent>,
) -> Result {
    let camera_transform = camera.single()?;
    let window = window.single()?;

    if ui_state.buttons.spawn_planet
        && let Some(click_position) = ui_state.context_menu_position
    {
        let spawn_position = mouse_pos_to_world(&click_position, camera_transform, &window.size());

        if let Ok(radius) = ui_state.radius_input.parse::<u32>() {
            planet_events.write(SpawnPlanetEvent {
                position: Position(spawn_position),
                radius: Radius(radius),
                r#type: PlanetType::Planet,
                seed: rand::rng().random(),
            });

            ui_state.context_menu_position = None;
        } else {
            // You dumb fuck
        }
    }

    Ok(())
}

fn mouse_pos_to_world(mouse_pos: &Vec2, camera_transform: &Transform, window_size: &Vec2) -> Vec2 {
    let abs_mouse_pos = mouse_pos - window_size / 2.0;

    camera_transform
        .transform_point(Vec3::new(abs_mouse_pos.x, -abs_mouse_pos.y, 0.0))
        .xy()
}
