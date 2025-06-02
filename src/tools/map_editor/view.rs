use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::*;
use robot_rumble::{core::physics::Position, entities::planet::Radius};

use crate::model::UiState;

pub struct ViewPlugin;
impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiContextPass,
            (render_context_menu, render_side_panel).chain(),
        );
    }
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

    egui::SidePanel::left("side_panel")
        .default_width(350.0)
        .show(ctx, move |ui| {
            if let Some(planet) = ui_state.focused_planet {
                let (mut position, mut radius) =
                    planet_query.get_mut(planet).expect("Invalid planets");

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

                ui.separator();
            }

            ui.heading("Map layout");

            ui.separator();

            let save_path_label = ui.label("Save file path");
            ui.text_edit_singleline(&mut ui_state.save_file_path)
                .labelled_by(save_path_label.id);
            ui.horizontal(|ui| {
                ui_state.buttons.save_map = ui.button("Save").clicked();
                ui_state.buttons.load_map = ui.button("Load").clicked();
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    Ok(())
}
