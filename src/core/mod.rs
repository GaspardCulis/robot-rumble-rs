use bevy::prelude::Plugin;

pub mod camera;
pub mod gravity;
pub mod physics;
pub mod spritesheet;
pub mod worldgen;

pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(camera::CameraPlugin)
            // .add_plugins(gravity::GravityPlugin)
            .add_plugins(physics::PhysicsPlugin)
            .add_plugins(spritesheet::AnimatedSpritePlugin)
            .add_plugins(worldgen::WorldgenPlugin);
    }
}
