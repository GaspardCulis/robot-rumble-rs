use bevy::dev_tools::picking_debug::{DebugPickingMode, DebugPickingPlugin};
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
    focused_planet: Option<Entity>,
    radius_input: String,
    buttons: ButtonsState,
}

#[derive(Default)]
struct ButtonsState {
    spawn_planet: bool,
}

fn main() {
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
        .init_resource::<UiState>()
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
                add_planet_pointer_observer,
                update_planet_radius,
                (drag_camera_view, handle_right_click).run_if(not(egui_wants_any_pointer_input)),
            ),
        )
        .add_systems(
            EguiContextPass,
            (
                render_context_menu,
                render_side_panel,
                handle_spawn_planet_button,
            )
                .chain(),
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

fn render_context_menu(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) -> Result {
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

fn render_side_panel(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut planet_query: Query<(&mut Position, &mut Radius)>,
    mut ui_state: ResMut<UiState>,
) -> Result {
    let ctx = contexts
        .try_ctx_mut()
        .ok_or(BevyError::from("Couldn't get egui context"))?;

    if let Some(planet) = ui_state.focused_planet {
        let (mut position, mut radius) = planet_query.get_mut(planet)?;

        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, move |ui| {
                ui.heading("Planet properties");

                ui.separator();

                let radius_label = ui.label("Position: ");
                ui.horizontal(|ui| {
                    let mut position_x_str: String = format!("{}", position.x);
                    let mut position_y_str: String = format!("{}", position.y);

                    ui.text_edit_singleline(&mut position_x_str)
                        .labelled_by(radius_label.id);
                    ui.text_edit_singleline(&mut position_y_str)
                        .labelled_by(radius_label.id);

                    position.x = position_x_str.parse().unwrap_or_default();
                    position.y = position_y_str.parse().unwrap_or_default();
                });

                ui.separator();

                let mut new_radius = radius.0;
                ui.add(egui::Slider::new(&mut new_radius, 10..=1000).text("Radius"));
                radius.set_if_neq(Radius(new_radius));

                ui.separator();

                if ui.button("Delete").clicked() {
                    commands.entity(planet).despawn();
                    ui_state.focused_planet = None;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.add(egui::Hyperlink::from_label_and_url(
                        "powered by egui",
                        "https://github.com/emilk/egui/",
                    ));
                });
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

fn add_planet_pointer_observer(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Radius), Added<Planet>>,
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

fn update_planet_radius(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(&Radius, &Mesh2d, &Children), (With<Planet>, Changed<Radius>)>,
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
