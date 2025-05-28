use crate::core::physics::{PhysicsSet, Position};
use bevy::prelude::*;
use bevy::render::mesh::Mesh;
use bevy::sprite::Material2dPlugin;
use bevy_common_assets::ron::RonAssetPlugin;

pub mod bumper;
pub mod grabber;
pub mod slingshot;
mod visuals;

use bevy_ggrs::GgrsSchedule;
use bumper::Bumper;
use grabber::Grabber;
use slingshot::{Slingshot, SlingshotVisual};
use visuals::{OrbitMaterial, generate_ring};

#[derive(Component)]
pub struct Satellite;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct SatelliteConfig {
    pub orbit_radius: f32,
    pub orbit_duration: f32,
    pub orbit_cooldown: f32,
    pub bump_radius: f32,
    pub bump_multiplier: f32,
    pub grabber_radius: f32,
}

#[derive(Resource)]
pub struct SatelliteConfigHandle(pub Handle<SatelliteConfig>);

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
            .add_plugins(RonAssetPlugin::<SatelliteConfig>::new(&[]))
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
            .add_systems(Startup, load_satellite_config)
            .add_systems(Update, handle_spawn_satellite)
            .add_plugins(slingshot::SlingshotPlugin)
            .add_plugins(bumper::BumperPlugin)
            .add_plugins(grabber::GrabberPlugin);
    }
}

fn load_satellite_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("config/satellites.ron");
    commands.insert_resource(SatelliteConfigHandle(handle));
}

fn handle_spawn_satellite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut events: EventReader<SpawnSatelliteEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<OrbitMaterial>>,
    config_handle: Res<SatelliteConfigHandle>,
    configs: Res<Assets<SatelliteConfig>>,
) {
    let Some(config) = configs.get(&config_handle.0) else {
        warn!("Satellite config not loaded yet");
        return;
    };
    let slingshot_active = asset_server.load("skins/satellite/working_slingshot.png");
    let slingshot_inactive = asset_server.load("skins/satellite/destroyed_slingshot.png");

    let bumper_texture = asset_server.load("skins/satellite/working_bumper.png");
    let grabber_texture = asset_server.load("skins/satellite/working_grabber.png");

    for event in events.read() {
        let mut entity = commands.spawn((
            Transform {
                translation: event.position.0.extend(0.0),
                scale: Vec3::splat(event.scale),
                ..default()
            },
            GlobalTransform::default(),
            Visibility::Visible,
            Satellite,
            event.kind,
        ));

        let child_transform = (
            Transform::from_translation(Vec3::new(130.0, 75.0, 0.0)),
            GlobalTransform::default(),
            Visibility::Visible,
        );

        match event.kind {
            SatelliteKind::Slingshot => {
                entity.insert((
                    Slingshot,
                    SlingshotVisual {
                        active: slingshot_active.clone(),
                        inactive: slingshot_inactive.clone(),
                    },
                ));

                entity.with_children(|parent| {
                    parent.spawn((
                        Sprite {
                            image: slingshot_active.clone(),
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
                            image: bumper_texture.clone(),
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
                            image: grabber_texture.clone(),
                            ..default()
                        },
                        child_transform,
                    ));
                });
            }
        }

        let (orbit_radius, base_color) = match event.kind {
            SatelliteKind::Slingshot => (
                config.orbit_radius + 100.0,
                LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            ),
            SatelliteKind::Bumper => (config.bump_radius, LinearRgba::new(1.0, 0.5, 0.0, 1.0)),
            SatelliteKind::Grabber => (
                config.grabber_radius + 50.0,
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
                GlobalTransform::default(),
            ));
        });
    }
}
