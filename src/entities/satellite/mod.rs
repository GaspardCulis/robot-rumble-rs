use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};
use bevy::sprite::Material2dPlugin;
use bevy_common_assets::ron::RonAssetPlugin;
use crate::core::physics::{Position};

pub mod bumper;
pub mod grabber;
pub mod graviton;
pub mod orbit_material;

use bumper::Bumper;
use grabber::Grabber;
use graviton::{GravitonMarker, GravitonVisual};
use orbit_material::OrbitMaterial;

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
    pub orbit_radius: f32,
    pub min_angular_speed: f32,
    pub orbit_duration: f32,
    pub orbit_cooldown: f32,
    pub decay_rate: f32,
    pub bump_radius: f32,
    pub bump_multiplier: f32,
    pub grabber_radius: f32,
}

#[derive(Resource)]
pub struct SatelliteConfigHandle(pub Handle<SatelliteConfig>);

#[derive(Resource, Default, Clone, Copy)]
struct OrbitTime(f32);

pub struct SatellitePlugin;
impl Plugin for SatellitePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<graviton::Orbited>()
            .add_plugins(RonAssetPlugin::<SatelliteConfig>::new(&[]))
            .add_plugins(Material2dPlugin::<OrbitMaterial>::default())
            .insert_resource(OrbitTime::default())
            .add_event::<SpawnSatelliteEvent>()
            .add_systems(Startup, load_satellite_config)
            .add_systems(
                Update,
                (
                    update_orbit_time,
                    update_orbit_material,
                    handle_spawn_satellite,
                ),
            )
            .add_plugins(graviton::GravitonPlugin)
            .add_plugins(bumper::BumperPlugin)
            .add_plugins(grabber::GrabberPlugin);
    }
}

pub fn load_satellite_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("config/satellites.ron");
    commands.insert_resource(SatelliteConfigHandle(handle));
}

fn update_orbit_time(mut orbit_time: ResMut<OrbitTime>, time: Res<Time>) {
    orbit_time.0 += time.delta_secs();
}

fn update_orbit_material(orbit_time: Res<OrbitTime>, mut materials: ResMut<Assets<OrbitMaterial>>) {
    for material in materials.iter_mut() {
        material.1.time = orbit_time.0;
    }
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
    let graviton_active = asset_server.load("skins/satellite/working_graviton.png");
    let graviton_inactive = asset_server.load("skins/satellite/destroyed_graviton.png");

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
                        child_transform.clone(),
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
                        child_transform.clone(),
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
                        child_transform.clone(),
                    ));
                });
            }
        }

        let (orbit_radius, base_color) = match event.kind {
            SatelliteKind::Graviton => (
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
            time: 0.0,
            base_color,
            saturation: 1.0,
            alpha: 0.6,
            _wasm_padding: 0.0,
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

pub fn generate_ring(inner_radius: f32, outer_radius: f32, resolution: usize) -> Mesh {
    let mut positions = Vec::with_capacity(resolution * 2);
    let mut uvs: Vec<Vec2> = Vec::with_capacity(resolution * 2);
    let mut indices = Vec::with_capacity(resolution * 6);

    for i in 0..resolution {
        let angle = i as f32 / resolution as f32 * std::f32::consts::TAU;
        let dir = Vec2::new(angle.cos(), angle.sin());
        positions.push((dir * outer_radius).extend(0.0));
        positions.push((dir * inner_radius).extend(0.0));

        uvs.push((dir * 0.5 + Vec2::splat(0.5)).into());
        uvs.push((dir * 0.5 + Vec2::splat(0.5)).into());
    }

    for i in 0..resolution {
        let i0 = (i * 2) as u32;
        let i1 = (i * 2 + 1) as u32;
        let i2 = ((i * 2 + 2) % (resolution * 2)) as u32;
        let i3 = ((i * 2 + 3) % (resolution * 2)) as u32;

        indices.extend_from_slice(&[i0, i2, i1, i2, i3, i1]);
    }

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; resolution * 2],
    );
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
