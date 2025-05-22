use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use super::physics::{PhysicsSet, Position};

#[derive(Component, Debug, Default, Reflect)]
pub enum CollisionShape {
    #[default]
    Point,
    /// Uses f32 internally to avoid casts
    Circle(f32),
}

#[derive(Component, Reflect)]
pub struct CollisionState<A, B> {
    pub collider: Option<Entity>,
    _data: PhantomData<(A, B)>,
}

pub struct CollisionPlugin<A, B>(PhantomData<(A, B)>);
impl<A, B> Plugin for CollisionPlugin<A, B>
where
    A: Component,
    B: Component,
{
    fn build(&self, app: &mut App) {
        app.register_type::<CollisionShape>()
            .register_required_components_with::<A, CollisionState<A, B>>(|| CollisionState {
                collider: None,
                _data: default(),
            })
            .add_systems(
                GgrsSchedule,
                check_collisions::<A, B>.in_set(PhysicsSet::Collision),
            );
    }
}

fn check_collisions<A, B>(
    mut query_a: Query<(&mut CollisionState<A, B>, &Position, &CollisionShape), With<A>>,
    query_b: Query<(Entity, &Position, &CollisionShape), With<B>>,
) where
    A: Component,
    B: Component,
{
    for (mut a_collision_state, a_position, a_shape) in query_a.iter_mut() {
        let collider = query_b
            .iter()
            // Sort by distance for determinism
            .sort_by::<&Position>(|x_position, y_position| {
                a_position
                    .distance_squared(x_position.0)
                    .total_cmp(&a_position.distance_squared(y_position.0))
            })
            // Get closest, others are irrelevant
            .next()
            .and_then(|(b_entity, b_position, b_shape)| {
                if a_shape.collides_with(a_position, b_shape, b_position) {
                    Some(b_entity)
                } else {
                    None
                }
            });

        if a_collision_state.collider != collider {
            // Update only if changed in order to properly trigger Changed<C> events
            a_collision_state.collider = collider;
        }
    }
}

impl<A, B> CollisionPlugin<A, B>
where
    A: Component,
    B: Component,
{
    pub fn new() -> Self {
        Self(default())
    }
}

impl CollisionShape {
    pub fn bounding_radius(&self) -> f32 {
        match self {
            CollisionShape::Point => 0.0,
            CollisionShape::Circle(radius) => *radius,
        }
    }

    pub fn collides_with(
        &self,
        self_position: &Position,
        other: &Self,
        other_position: &Position,
    ) -> bool {
        let distance_squared = self_position.distance_squared(other_position.0);
        let radius_squared = (self.bounding_radius() + other.bounding_radius()).powi(2);

        distance_squared <= radius_squared
    }
}
