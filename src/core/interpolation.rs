use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::{GgrsSchedule, GgrsTime};

use super::physics::{PhysicsSet, Position, Rotation};

#[derive(Component, Default, Reflect)]
pub struct Interpolate;

/// Duration since last GGRS frame
#[derive(Resource, Default, Deref, DerefMut)]
struct LastTickDuration(Duration);

#[derive(Component, Default, Reflect)]
struct InterpolationCache {
    old: Transform,
    target: Transform,
}

pub struct InterpolationPlugin;
impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interpolate>()
            .register_type::<InterpolationCache>()
            .init_resource::<LastTickDuration>()
            .add_systems(
                GgrsSchedule,
                (reset_last_tick_duration, update_interpolation_cache).after(PhysicsSet::Movement),
            )
            .add_systems(
                Update,
                (
                    setup_interpolation_cache,
                    update_last_tick_duration,
                    interpolate_transforms,
                    update_normal_transforms,
                )
                    .chain(),
            );
    }
}

fn reset_last_tick_duration(mut last: ResMut<LastTickDuration>) {
    last.0 = Duration::default();
}

fn update_last_tick_duration(mut last: ResMut<LastTickDuration>, time: Res<Time>) {
    last.0 += time.delta();
}

fn setup_interpolation_cache(
    mut commands: Commands,
    mut query: Query<
        (Entity, &mut Transform, &Position, Option<&Rotation>),
        (Added<Transform>, With<Interpolate>),
    >,
) {
    query
        .iter_mut()
        .for_each(|(entity, mut transform, position, rotation)| {
            // Immediatly update transform in order to not have to wait for next tick
            // Yields better visual results
            transform.translation.x = position.x;
            transform.translation.y = position.y;

            if let Some(rotation) = rotation {
                transform.rotation = Quat::from_rotation_z(rotation.0);
            }

            commands
                .entity(entity)
                .insert(InterpolationCache::from(*transform));
        });
}

fn update_interpolation_cache(
    mut query: Query<(&mut InterpolationCache, &Position, Option<&Rotation>)>,
) {
    query
        .iter_mut()
        .for_each(|(mut cache, position, rotation)| {
            cache.old = cache.target;

            cache.target.translation.x = position.x;
            cache.target.translation.y = position.y;

            if let Some(rotation) = rotation {
                cache.target.rotation = Quat::from_rotation_z(rotation.0);
            }
        });
}

/// Performs transform updates on entities that do not require interpolation
fn update_normal_transforms(
    mut query: Query<(&mut Transform, &Position, Option<&Rotation>), Without<InterpolationCache>>,
) {
    // `for_each` is more performant than a standard for loop
    query
        .iter_mut()
        .for_each(|(mut transform, position, rotation)| {
            transform.translation.x = position.x;
            transform.translation.y = position.y;

            if let Some(rotation) = rotation {
                transform.rotation = Quat::from_rotation_z(rotation.0);
            }
        });
}

// Inspired by https://bevyengine.org/examples/movement/physics-in-fixed-timestep/
fn interpolate_transforms(
    mut query: Query<(&mut Transform, &InterpolationCache)>,
    ggrs_time: Res<Time<GgrsTime>>,
    last: Res<LastTickDuration>,
) {
    let alpha = last.as_secs_f32() / ggrs_time.delta_secs();

    for (mut transform, cache) in query.iter_mut() {
        transform.translation = cache.old.translation.lerp(cache.target.translation, alpha);

        transform.rotation = cache.old.rotation.lerp(cache.target.rotation, alpha);
    }
}

impl From<Transform> for InterpolationCache {
    fn from(value: Transform) -> Self {
        Self {
            old: value,
            target: value,
        }
    }
}
