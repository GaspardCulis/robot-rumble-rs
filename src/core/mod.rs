use bevy::prelude::{App, Plugin};

pub mod audio;
pub mod background;
pub mod camera;
pub mod collision;
pub mod gravity;
pub mod inputs;
pub mod physics;
pub mod worldgen;
pub struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(background::BackgroundPlugin)
            .add_plugins(camera::CameraPlugin)
            .add_plugins(audio::GameAudioPlugin)
            .add_plugins(gravity::GravityPlugin)
            .add_plugins(inputs::InputsPlugin)
            .add_plugins(physics::PhysicsPlugin)
            .add_plugins(worldgen::WorldgenPlugin);
    }
}
