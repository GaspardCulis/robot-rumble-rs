use std::marker::PhantomData;

use bevy::{math::FloatPow as _, prelude::*};
use bevy_ggrs::GgrsSchedule;

use super::physics::{PhysicsSet, Position};

#[derive(Component, Clone, Debug, Default, Reflect)]
pub enum CollisionShape {
    #[default]
    Point,
    /// Uses f32 internally to avoid casts
    Circle(f32),
}

#[derive(Component, Clone, Reflect)]
pub struct CollisionState<A, B> {
    pub closest: Option<Entity>,
    pub collides: bool,
    _data: PhantomData<(A, B)>,
}

/// Adds systems that checks collisions between entities with component `A`, and entities with component `B`.
/// Entity `A` gets added the `CollisionState` component to get collisions notified.
pub struct CollisionPlugin<A, B>(PhantomData<(A, B)>);
impl<A, B> Plugin for CollisionPlugin<A, B>
where
    A: Component,
    B: Component,
{
    fn build(&self, app: &mut App) {
        app.register_type::<CollisionShape>()
            .register_required_components_with::<A, CollisionState<A, B>>(|| CollisionState {
                closest: None,
                collides: false,
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
    query_a
        .par_iter_mut()
        .for_each(|(mut a_collision_state, a_position, a_shape)| {
            let (closest, collides) = query_b
                .iter()
                // Find the closest entity using min_by for O(n) complexity
                .min_by(|(_, x_pos, x_shape), (_, y_pos, y_shape)| {
                    let x_dist =
                        x_pos.distance_squared(a_position.0) - x_shape.bounding_radius().squared();
                    let y_dist =
                        y_pos.distance_squared(a_position.0) - y_shape.bounding_radius().squared();

                    x_dist.total_cmp(&y_dist)
                })
                .map_or_else(
                    || (None, false),
                    |(b_entity, b_position, b_shape)| {
                        let collides = a_shape.collides_with(a_position, b_shape, b_position);
                        (Some(b_entity), collides)
                    },
                );

            if a_collision_state.closest != closest {
                // Update only if changed in order to properly trigger Changed<C> events
                a_collision_state.closest = closest;
            }
            if a_collision_state.collides != collides {
                // Same
                a_collision_state.collides = collides;
            }
        });
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
        let radius_squared = (self.bounding_radius() + other.bounding_radius()).squared();

        distance_squared <= radius_squared
    }
}
