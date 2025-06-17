use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::level::limit::MapLimit;

use super::physics::PhysicsSet;

/// Extra space so camera sees edges without clipping
const EDGE_PADDING: f32 = 200.0;
/// How fast camera zooms in/out
const ZOOM_SPEED: f32 = 3.0;
/// How fast camera follows
const FOLLOW_SPEED: f32 = 5.0;
/// What fraction of the world is seen at least
const MIN_WORLD_FRACTION: f32 = 0.2; // i.e. 20%

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
    targets: Query<&Transform, With<CameraFollowTarget>>,
    limit: Option<Res<MapLimit>>,
    time: Res<Time>,
) -> Result {
    if targets.is_empty() {
        // Not having target is not an error
        return Ok(());
    };
    let mut camera_transform = camera.single_mut()?;
    let window = window.single()?;
    let screen_size = window.size();

    let positions: Vec<Vec2> = targets.iter().map(|t| t.translation.xy()).collect();
    let center = positions.iter().copied().sum::<Vec2>() / positions.len() as f32;

    // Bounding box for players
    let max_dist = positions
        .iter()
        .map(|&pos| pos.distance(center))
        .fold(0.0, f32::max);
    // Zoom scale
    let mut target_scale = ((max_dist + EDGE_PADDING) * 2.0) / screen_size.min_element();
    // Adjust scale with world bounds
    if let Some(limit) = limit {
        let world_extent = (limit.radius + EDGE_PADDING) * 2.0;
        let min_scale = (world_extent * MIN_WORLD_FRACTION) / screen_size.min_element();
        let max_scale = world_extent / screen_size.min_element();
        target_scale = target_scale.clamp(min_scale, max_scale);
    }
    // Zoom transition
    let current_scale = camera_transform.scale;
    camera_transform.scale =
        current_scale.lerp(Vec3::splat(target_scale), time.delta_secs() * ZOOM_SPEED);
    // Follow center
    let target_pos = Vec3::new(center.x, center.y, camera_transform.translation.z);
    camera_transform.translation = camera_transform
        .translation
        .lerp(target_pos, time.delta_secs() * FOLLOW_SPEED);

    Ok(())
}
