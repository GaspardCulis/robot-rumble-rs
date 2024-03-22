use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera2dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::hex("#171711").unwrap()),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    }
}
