use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::utils::math;

use super::physics::PhysicsSet;

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
    let offset = (cursor_position / screen_size - 0.5) * Vec2::new(1., -1.) * max_cursor_offset;

    let dest = target_transform.translation.xy() + offset * camera_transform.scale.xy();

    camera_transform.translation = math::lerp(
        camera_transform.translation,
        Vec3::new(dest.x, dest.y, 0.),
        time.delta_secs() * 5.,
    );

    Ok(())
}
