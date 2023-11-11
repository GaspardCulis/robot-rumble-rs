use bevy::prelude::*;
use components::physics::{Mass, Position, Velocity};

mod components;
mod entities;
mod systems;

fn add_players(mut commands: Commands) {
    commands.spawn((
        Position { x: 0., y: 0. },
        Velocity { x: 0., y: 0. },
        Mass(0),
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_players)
        .add_systems(Update, systems::gravity::apply_forces)
        .run();
}
