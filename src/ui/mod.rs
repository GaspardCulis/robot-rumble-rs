use bevy::app::Plugin;

mod hud;

pub struct UiPlugins;
impl Plugin for UiPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins(hud::HudPlugin);
    }
}
