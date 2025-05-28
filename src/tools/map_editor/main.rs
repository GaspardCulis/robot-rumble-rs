use bevy::{
    input::mouse::{AccumulatedMouseMotion, MouseWheel},
    prelude::*,
};

use bevy_inspector_egui::{
    bevy_egui::{input::egui_wants_any_pointer_input, *},
    quick::WorldInspectorPlugin,
};
use robot_rumble::{
    core::{
        physics::Position,
        worldgen::{WorldgenConfig, WorldgenConfigHandle},
    },
    entities::planet::*,
    *,
};

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
                drag_camera_view.run_if(not(egui_wants_any_pointer_input)),
            ),
        )
        .add_systems(EguiContextPass, ui_example_system)
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
        seed: rand::Rng::random(&mut rand::rng()),
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

#[derive(Default, Resource)]
struct UiState {
    label: String,
    value: f32,
    inverted: bool,
    egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
}

fn ui_example_system(mut contexts: EguiContexts, mut ui_state: ResMut<UiState>) {
    let ctx = if let Some(ctx) = contexts.try_ctx_mut() {
        ctx
    } else {
        return;
    };

    let mut clicked = false;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                clicked = ui.button("Click").clicked();
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_window_open, "Window Is Open");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    if clicked {
        println!("Clicko");
    }
}
