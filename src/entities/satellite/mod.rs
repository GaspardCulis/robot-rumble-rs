use crate::core::physics::{PhysicsSet, Position};
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::Material2dPlugin;

pub mod assets;
pub mod bumper;
pub mod grabber;
pub mod slingshot;
mod visuals;

use assets::{SatelliteAssets, SatelliteConfig};
use bevy_ggrs::GgrsSchedule;
use bumper::Bumper;
use grabber::Grabber;
use slingshot::{Slingshot, SlingshotVisual};
use visuals::{OrbitMaterial, generate_ring};

#[derive(Component)]
#[require(Visibility)]
pub struct Satellite;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, serde::Serialize, serde::Deserialize)]
pub enum SatelliteKind {
    Slingshot,
    Bumper,
    Grabber,
}

#[derive(Event, Debug, Clone)]
pub struct SpawnSatelliteEvent {
    pub position: Position,
    pub scale: f32,
    pub kind: SatelliteKind,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum SatelliteSet {
    Slingshot,
    Bumper,
    Grabber,
}

pub struct SatellitePlugin;
impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<slingshot::Orbited>()
            .add_plugins(Material2dPlugin::<OrbitMaterial>::default())
            .configure_sets(
                GgrsSchedule,
                (
                    SatelliteSet::Slingshot,
                    SatelliteSet::Bumper,
                    SatelliteSet::Grabber,
                )
                    .chain()
                    .in_set(PhysicsSet::Interaction),
            )
            .add_event::<SpawnSatelliteEvent>()
            .add_systems(
                Update,
                handle_spawn_satellite.run_if(resource_exists::<SatelliteAssets>),
            )
            .add_plugins(slingshot::SlingshotPlugin)
            .add_plugins(bumper::BumperPlugin)
            .add_plugins(grabber::GrabberPlugin);
    }
}

fn handle_spawn_satellite(
    mut commands: Commands,
    mut events: EventReader<SpawnSatelliteEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OrbitMaterial>>,
    configs: Res<Assets<SatelliteConfig>>,
    assets: Res<SatelliteAssets>,
) {
    let Some(config) = configs.get(&assets.config) else {
        warn!("Satellite config not loaded yet");
        return;
    };

    for event in events.read() {
        let mut entity = commands.spawn((
            Satellite,
            Transform::from_scale(Vec3::splat(event.scale)),
            event.position.clone(),
        ));

        let child_transform = Transform::from_translation(Vec3::new(130.0, 75.0, 0.0));

        match event.kind {
            SatelliteKind::Slingshot => {
                entity.insert((
                    Slingshot,
                    SlingshotVisual {
                        active: assets.working_graviton.clone(),
                        inactive: assets.destroyed_graviton.clone(),
                    },
                ));

                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: assets.working_graviton.clone(),
                            ..default()
                        },
                        child_transform,
                    ));
                });
            }
            SatelliteKind::Bumper => {
                entity.insert(Bumper);
                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: assets.working_bumper.clone(),
                            ..default()
                        },
                        child_transform,
                    ));
                });
            }
            SatelliteKind::Grabber => {
                entity.insert(Grabber);
                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: assets.working_grabber.clone(),
                            ..default()
                        },
                        child_transform,
                    ));
                });
            }
        }

        let (orbit_radius, base_color) = match event.kind {
            SatelliteKind::Slingshot => (
                config.slingshot.orbit_radius + 100.0,
                LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            ),
            SatelliteKind::Bumper => (config.bumper.radius, LinearRgba::new(1.0, 0.5, 0.0, 1.0)),
            SatelliteKind::Grabber => (
                config.grabber.radius + 50.0,
                LinearRgba::new(0.0, 1.0, 0.0, 1.0),
            ),
        };

        let orbit_material_handle = materials.add(OrbitMaterial {
            base_color,
            saturation: 1.0,
            alpha: 0.6,
            _wasm_padding: default(),
        });

        let ring_thickness = 5.0;
        let orbit_ring = meshes.add(generate_ring(
            orbit_radius - ring_thickness,
            orbit_radius,
            64,
        ));

        entity.with_children(|parent| {
            parent.spawn((
                Mesh2d::from(orbit_ring),
                MeshMaterial2d(orbit_material_handle),
                Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
            ));
        });
    }
}
