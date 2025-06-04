use bevy::prelude::*;
use bevy_ggrs::GgrsTime;

use super::{
    camera::VisualsSet,
    physics::{Position, Rotation},
};

#[derive(Component, Default, Reflect)]
pub struct Interpolate;

pub struct InterpolationPlugin;
impl Plugin for InterpolationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Interpolate>().add_systems(
            Update,
            (setup_initial_transform, interpolate_transforms)
                .chain()
                .in_set(VisualsSet::Interpolation),
        );
    }
}

/// Immediatly updates transform for entities with added Transform, Posision, or Interpolate.
/// Avoids artifact interpolations starting from `Transform::default`
fn setup_initial_transform(
    mut query: Query<
        (&mut Transform, &Position, Option<&Rotation>),
        (
            Or<(Changed<Interpolate>, Added<Transform>, Added<Position>)>,
            With<Interpolate>,
        ),
    >,
) {
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
    mut query: Query<(
        &mut Transform,
        &Position,
        Option<&Rotation>,
        Has<Interpolate>,
    )>,
    ggrs_time: Res<Time<GgrsTime>>,
    time: Res<Time>,
) {
    let alpha = time.delta_secs() / ggrs_time.delta_secs();

    for (mut transform, position, rotation, interpolate) in query.iter_mut() {
        let rendered_transform = if interpolate {
            transform.translation.xy().lerp(position.0, alpha)
        } else {
            position.0
        };
        transform.translation.x = rendered_transform.x;
        transform.translation.y = rendered_transform.y;

        if let Some(rotation) = rotation {
            let rotation_quat = Quat::from_rotation_z(rotation.0);

            transform.rotation = if interpolate {
                transform.rotation.lerp(rotation_quat, alpha)
            } else {
                rotation_quat
            };
        }
    }
}
