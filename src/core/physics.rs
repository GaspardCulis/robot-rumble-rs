use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position(pub Vec2);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug)]
pub struct Rotation(pub f32);

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
        app.add_systems(Update, update_spatial_bundles);
    }
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_seconds()
    }
}

fn update_spatial_bundles(mut query: Query<(&mut Transform, &Position, Option<&Rotation>)>) {
    for (mut transform, position, rotation) in query.iter_mut() {
        transform.translation.x = position.0.x;
        transform.translation.y = position.0.y;

        if rotation.is_some() {
            transform.rotation = Quat::from_rotation_z(rotation.unwrap().0);
        }
    }
}
