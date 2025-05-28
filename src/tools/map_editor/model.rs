use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct UiState {
    pub context_menu_position: Option<Vec2>,
    pub focused_planet: Option<Entity>,
    pub radius_input: String,
    pub buttons: ButtonsState,
}

#[derive(Default)]
pub struct ButtonsState {
    pub spawn_planet: bool,
}

pub struct ModelPlugin;
impl Plugin for ModelPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>();
    }
}
