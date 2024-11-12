use bevy::prelude::{App, Plugin};

pub mod camera;

pub struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(camera::CameraPlugin);
    }
}
