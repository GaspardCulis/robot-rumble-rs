use bevy::prelude::Plugin;

use self::{
    camera::CameraPlugin, gravity::GravityPlugin, physics::PhysicsPlugin,
    spritesheet::AnimatedSpritePlugin,
};

pub mod camera;
pub mod gravity;
pub mod physics;
pub mod spritesheet;

pub struct CorePlugins;

impl Plugin for CorePlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(CameraPlugin);
        app.add_plugins(PhysicsPlugin);
        app.add_plugins(GravityPlugin);
        app.add_plugins(AnimatedSpritePlugin);
    }
}
