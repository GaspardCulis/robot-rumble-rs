use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui::RichText};
use robot_rumble::{
    core::physics::Position,
    entities::{planet::Radius, satellite},
};

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
        egui::Window::new("Spawn entity")
            .collapsible(false)
            .fixed_pos((position.x, position.y))
            .fixed_size([200.0, 300.0])
            .show(ctx, |ui| {
                ui.heading("Planet");
                ui.horizontal(|ui| {
                    let radius_label = ui.label("Radius: ");
                    ui.text_edit_singleline(&mut ui_state.radius_input)
                        .labelled_by(radius_label.id);
                });
                ui_state.buttons.spawn_planet = ui.button("Spawn planet").clicked();

                ui.separator();

                ui.heading("Satellite");
                ui.label("Kind");
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut ui_state.satellite_kind_input,
                        satellite::SatelliteKind::Bumper,
                        "Bumper",
                    );
                    ui.selectable_value(
                        &mut ui_state.satellite_kind_input,
                        satellite::SatelliteKind::Slingshot,
                        "Slingshot",
                    );
                    ui.selectable_value(
                        &mut ui_state.satellite_kind_input,
                        satellite::SatelliteKind::Grabber,
                        "Grabber",
                    );
                });
                ui_state.buttons.spawn_satellite = ui.button("Spawn satellite").clicked();
            });
    }

    Ok(())
}

fn render_side_panel(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut planet_query: Query<(&mut Position, Option<&mut Radius>)>,
    mut ui_state: ResMut<UiState>,
) -> Result {
    let ctx = contexts
        .try_ctx_mut()
        .ok_or(BevyError::from("Couldn't get egui context"))?;

    egui::SidePanel::left("side_panel")
        .default_width(350.0)
        .show(ctx, move |ui| {
            if let Some(planet) = ui_state.focused_entity {
                let Ok((mut position, radius)) = planet_query.get_mut(planet) else {
                    ui_state.error_message = Some(format!(
                        "Invalid focused entity: {:?}",
                        ui_state.focused_entity
                    ));
                    return;
                };

                ui.heading("Entity properties");

                ui.separator();

                let radius_label = ui.label("Position: ");
                ui.horizontal(|ui| {
                    let mut position_x_str: String = format!("{}", position.x);
                    let mut position_y_str: String = format!("{}", position.y);

                    ui.text_edit_singleline(&mut position_x_str)
                        .labelled_by(radius_label.id);
                    ui.text_edit_singleline(&mut position_y_str)
                        .labelled_by(radius_label.id);

                    if let Ok(x) = position_x_str.parse()
                        && let Ok(y) = position_y_str.parse()
                    {
                        position.x = x;
                        position.y = y;
                    } else {
                        ui_state.error_message = Some(format!(
                            "Failed to parse position (x: {position_x_str}, y: {position_y_str})"
                        ));
                    }
                });

                if let Some(mut radius) = radius {
                    ui.separator();

                    let mut new_radius = radius.0;
                    ui.add(egui::Slider::new(&mut new_radius, 10..=1000).text("Radius"));
                    radius.set_if_neq(Radius(new_radius));
                }

                ui.separator();

                if ui.button("Delete").clicked() {
                    commands.entity(planet).despawn();
                    ui_state.focused_entity = None;
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
                if let Some(error_msg) = &ui_state.error_message {
                    ui.label(RichText::new(error_msg).color(egui::Color32::RED));
                };

                ui.add(egui::Hyperlink::from_label_and_url(
                    "powered by egui",
                    "https://github.com/emilk/egui/",
                ));
            });
        });

    Ok(())
}
