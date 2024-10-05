use bevy::prelude::*;

use crate::utils::math;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::Srgba(
                        Srgba::hex("#171711").unwrap(),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .add_systems(Update, camera_movement);
    }
}

#[derive(Component)]
pub struct CameraFollowTarget;

fn camera_movement(
    mut camera: Query<&mut Transform, (With<Camera2d>, Without<CameraFollowTarget>)>,
    window: Query<&Window>,
    target: Query<&Transform, With<CameraFollowTarget>>,
    time: Res<Time>,
) {
    let mut camera_transform = camera.single_mut();
    let window = window.single();
    let target_transform = target.single();

    let screen_size = window.size();
    let cursor_position = window.cursor_position().unwrap_or(screen_size / 2.);

    let mut dest = target_transform.translation.xy();
    // Add mouse deviation
    dest += (cursor_position / screen_size - 0.5) * Vec2::new(-1., 1.) * -500.;

    camera_transform.translation = math::lerp(
        camera_transform.translation,
        Vec3::new(dest.x, dest.y, 0.),
        time.delta_seconds(),
    );
}
