use bevy::{math::FloatPow, prelude::*};
use bevy_ggrs::GgrsSchedule;

use crate::{level::limit::MapLimit, utils::math};

use super::physics::PhysicsSet;

/// Start biasing camera toward center past this world radius fraction
const EDGE_BIAS_THRESHOLD: f32 = 0.65;
/// Extra space so camera sees world edges without clipping
const EDGE_PADDING: f32 = 200.0;
/// Camera translation speed increase
const MAX_EXTRA_SPEED: f32 = 10.0;
/// How strongly the camera biases toward center near the edge
const BIAS_STRENGTH_FACTOR: f32 = 0.3;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn((Camera2d, Msaa::Off, Transform::from_scale(Vec3::splat(1.4))));
        })
        .add_systems(GgrsSchedule, camera_movement.after(PhysicsSet::Movement));
    }
}

#[derive(Component)]
pub struct CameraFollowTarget;

fn camera_movement(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraFollowTarget>)>,
    window: Query<&Window>,
    target: Query<&Transform, With<CameraFollowTarget>>,
    limit: Option<Res<MapLimit>>,
    time: Res<Time>,
) -> Result {
    if target.is_empty() {
        // Not having target is not an error
        return Ok(());
    };

    let mut camera_transform = camera.single_mut()?;
    let window = window.single()?;
    let target_transform = target.single()?;

    let screen_size = window.size();
    let cursor_position = window.cursor_position().unwrap_or(screen_size / 2.);
    let max_cursor_offset = screen_size * 0.2;
    let mut offset = (cursor_position / screen_size - 0.5) * Vec2::new(1., -1.) * max_cursor_offset;
    let player_pos = target_transform.translation.xy();

    let mut dest = player_pos;

    let mut follow_speed = 5.0;

    if let Some(limit) = limit {
        // Calculate player's bias to worlds's center
        let world_center = Vec2::ZERO;
        let dist_from_center = player_pos.distance(world_center);
        let bias_factor = ((dist_from_center - EDGE_BIAS_THRESHOLD * limit.radius)
            / ((1.0 - EDGE_BIAS_THRESHOLD) * limit.radius))
            .clamp(0.0, 1.0)
            .squared();
        dest = dest.lerp(world_center, bias_factor * BIAS_STRENGTH_FACTOR);

        // Clamp camera on rectangular bounds
        let bounds = limit.radius + EDGE_PADDING;
        let min_bound = Vec2::new(-bounds, -bounds);
        let max_bound = Vec2::new(bounds, bounds);
        let half_screen = (screen_size * 0.5) * camera_transform.scale.xy();

        offset *= 1.0 - bias_factor;
        dest = dest.clamp(min_bound + half_screen, max_bound - half_screen);

        // Adjust follow speed on bounds
        follow_speed += MAX_EXTRA_SPEED * bias_factor;
    }

    dest += offset;

    camera_transform.translation = math::lerp(
        camera_transform.translation,
        Vec3::new(dest.x, dest.y, 0.),
        time.delta_secs() * follow_speed,
    );

    Ok(())
}
