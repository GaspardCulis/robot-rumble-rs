use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::GameState;

#[derive(Component, Debug, Default, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[require(Transform)]
pub struct Position(pub Vec2);

#[derive(Component, Debug, Default, Reflect, Clone, PartialEq, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Debug, Default, Reflect, Clone, PartialEq, Deref, DerefMut)]
pub struct Rotation(pub f32);

#[derive(Bundle, Default)]
pub struct PhysicsBundle {
    pub position: Position,
    pub velocity: Velocity,
    pub rotation: Rotation,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSet {
    /// Where player inputs are processed
    Player,
    /// Where Velocity gets updated
    Gravity,
    /// Where entities interact with each other
    Interaction,
    /// Where Position gets updated
    Movement,
    /// Where collision detection systems are run
    Collision,
}

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Position>()
            .register_type::<Velocity>()
            .register_type::<Rotation>()
            .configure_sets(
                GgrsSchedule,
                (
                    PhysicsSet::Player,
                    PhysicsSet::Gravity,
                    PhysicsSet::Interaction,
                    PhysicsSet::Movement,
                    PhysicsSet::Collision,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(GgrsSchedule, update_position.in_set(PhysicsSet::Movement))
            .add_systems(
                Update,
                update_spatial_bundles
                    .in_set(PhysicsSet::Movement)
                    .after(update_position),
            );
    }
}

fn update_position(mut query: Query<(&mut Position, &Velocity)>, time: Res<Time>) {
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

impl std::ops::Sub for Position {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
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

impl Eq for &Position {}

impl PartialOrd for &Position {
    /// Compares distance from origin
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for &Position {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length_squared().total_cmp(&other.length_squared())
    }
}
