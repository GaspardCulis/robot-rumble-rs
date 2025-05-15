use bevy::prelude::*;
use crate::core::physics::Position;

use super::graviton::{GravitonMarker, GravitonVisual};
use super::bumper::Bumper;


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
#[derive(serde::Deserialize, Asset, TypePath)]
pub struct SatelliteConfig {
    // Graviton
    pub orbit_radius: f32,
    pub min_angular_speed: f32,
    pub orbit_duration: f32,
    pub orbit_cooldown: f32,
    pub decay_rate: f32,

    // Bumper
    pub bump_radius: f32,
    pub bump_multiplier: f32,
}

#[derive(Resource)]
pub struct SatelliteConfigHandle(pub Handle<SatelliteConfig>);

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
    let graviton_active = asset_server.load("skins/satellite/working_graviton.png");
    let graviton_inactive = asset_server.load("skins/satellite/destroyed_graviton.png");

    let bumper_texture = asset_server.load("skins/satellite/working_bumper.png");

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

        match event.kind {
            SatelliteKind::Graviton => {
                entity.insert((
                    GravitonMarker,
                    GravitonVisual {
                        active: graviton_active.clone(),
                        inactive: graviton_inactive.clone(),
                    },
                ));

                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: graviton_active.clone(),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(100.0, 75.0, 0.0)),
                        GlobalTransform::default(),
                        Visibility::Visible,
                    ));
                });
            }

            SatelliteKind::Bumper => {
                entity.insert(Bumper);

                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: bumper_texture.clone(),
                            ..default()
                        },
                        Transform::from_translation(Vec3::new(100.0, 75.0, 0.0)),
                        GlobalTransform::default(),
                        Visibility::Visible,
                    ));
                });
            }

            SatelliteKind::Grabber => {
                warn!("Grabber not implemented yet.");
            }
        }
    }
}


pub fn load_satellite_config(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let handle = asset_server.load("config/satellites.ron");
    commands.insert_resource(SatelliteConfigHandle(handle));
}