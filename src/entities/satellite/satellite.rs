use bevy::prelude::*;
use crate::core::physics::Position;

use super::graviton::{GravitonVisual};


#[derive(Component)]
pub struct Satellite;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum SatelliteKind {
    Graviton,
    Bumper,
    Grabber,
}

#[derive(Event, Debug, Clone)]
pub struct SpawnSatelliteEvent {
    pub position: Position,
    pub scale: f32,
    pub kind: SatelliteKind,
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
    let texture_active = asset_server.load("skins/satellite/working_graviton.png");
    let texture_inactive = asset_server.load("skins/satellite/destroyed_graviton.png");

    for event in events.read() {
        let mut entity = commands.spawn((
            Transform {
                translation: event.position.0.extend(5.0),
                scale: Vec3::splat(event.scale),
                ..default()
            },
            GlobalTransform::default(),
            Visibility::Visible,
            Satellite,
            event.kind,
        ));

        if event.kind == SatelliteKind::Graviton {
            entity.insert((
                GravitonVisual {
                    active: texture_active.clone(),
                    inactive: texture_inactive.clone(),
                },
            ));
        }

        entity.with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: texture_active.clone(),
                    ..default()
                },
                Transform::from_translation(Vec3::new(100.0, 75.0, 0.0)),
                GlobalTransform::default(),
                Visibility::Visible,
            ));
        });

    }
}
