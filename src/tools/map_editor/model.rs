use bevy::prelude::*;

#[derive(Resource)]
pub struct UiState {
    pub context_menu_position: Option<Vec2>,
    pub focused_planet: Option<Entity>,
    pub radius_input: String,
    pub save_file_path: String,
    pub buttons: ButtonsState,
}

#[derive(Default)]
pub struct ButtonsState {
    pub spawn_planet: bool,
    pub save_map: bool,
    pub load_map: bool,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            context_menu_position: None,
            focused_planet: None,
            radius_input: "100".to_string(),
            save_file_path: "map.ron".to_string(),
            buttons: ButtonsState::default(),
        }
    }
}
