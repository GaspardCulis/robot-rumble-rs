use bevy::prelude::*;
use crate::core::physics::Position;

#[derive(Component)]
pub struct Satellite;

#[derive(Event, Debug, Clone)]
pub struct SpawnSatelliteEvent {
    pub position: Position,
    pub scale: f32,
}

pub struct SatellitePlugin;
impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnSatelliteEvent>()
            .add_systems(Update, handle_spawn_satellite);
    }
}

fn handle_spawn_satellite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnSatelliteEvent>,
) {
    let texture_handle = asset_server.load("skins/satellite/working_satellite.png");

    for event in events.read() {
        commands.spawn((
            Sprite {
                image: texture_handle.clone(),
                ..Default::default()
            },
            Transform {
                translation: event.position.0.extend(5.0),
                scale: Vec3::splat(event.scale),
                ..Default::default()
            },
            Visibility::Visible,
            Satellite,
        ));
    }
}
