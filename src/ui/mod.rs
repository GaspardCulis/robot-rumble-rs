use bevy::app::Plugin;

mod hud;
mod menus;

pub use menus::Screen;

pub struct UiPlugins;
impl Plugin for UiPlugins {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_plugins((hud::HudPlugin, menus::MenusPlugin));
    }
}
