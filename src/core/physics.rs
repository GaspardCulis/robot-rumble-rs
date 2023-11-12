use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        todo!()
    }
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds()
    }
}
