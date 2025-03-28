use bevy::prelude::*;
use lightyear::prelude::FixedUpdateSet;
use serde::{Deserialize, Serialize};

use crate::network::TICK_DURATION;

#[derive(Component, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Component, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug, Reflect, Clone, PartialEq, Serialize, Deserialize, Deref, DerefMut)]
pub struct Rotation(pub f32);

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicsSet;

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>()
            .register_type::<Velocity>()
            .register_type::<Rotation>()
            .insert_resource(Time::new_with(TICK_DURATION))
            .configure_sets(FixedUpdate, PhysicsSet.in_set(FixedUpdateSet::TickUpdate))
            .add_systems(FixedUpdate, update_position.in_set(PhysicsSet))
            .add_systems(Last, update_spatial_bundles.in_set(PhysicsSet));
    }
}

pub fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
    for (mut position, velocity) in query.iter_mut() {
        position.0 += velocity.0 * time.delta_secs()
    }
}

fn update_spatial_bundles(mut query: Query<(&mut Transform, &Position, Option<&Rotation>)>) {
    for (mut transform, position, rotation) in query.iter_mut() {
        transform.translation.x = position.x;
        transform.translation.y = position.y;

        if rotation.is_some() {
            transform.rotation = Quat::from_rotation_z(rotation.unwrap().0);
        }
    }
}

// Chore ops implementations

impl std::ops::Add for Position {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl std::ops::Mul<f32> for &Position {
    type Output = Position;

    fn mul(self, rhs: f32) -> Self::Output {
        Position(self.0 * rhs)
    }
}

impl std::ops::Add for Rotation {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl std::ops::Mul<f32> for &Rotation {
    type Output = Rotation;

    fn mul(self, rhs: f32) -> Self::Output {
        Rotation(self.0 * rhs)
    }
}
