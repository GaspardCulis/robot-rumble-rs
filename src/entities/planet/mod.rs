use crate::core::{gravity::Mass, physics::Position};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use std::f64::consts::PI;

mod planet_materials;

const DEFAULT_RADIUS: u32 = 128;

#[derive(Component)]
pub struct Planet;

#[derive(Component)]
pub struct Radius(pub u32);

#[derive(Bundle)]
struct PlanetBundle {
    marker: Planet,
    position: Position,
    radius: Radius,
    mass: Mass,
}

impl Default for PlanetBundle {
    fn default() -> Self {
        Self {
            marker: Planet,
            position: Position(Vec2::ZERO),
            radius: Radius(DEFAULT_RADIUS),
            mass: Mass(radius_to_mass(DEFAULT_RADIUS)),
        }
    }
}

pub struct PlanetPlugin;

impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(planet_materials::PlanetMaterialsPlugin)
            .add_systems(Startup, spawn_planet);
    }
}

fn spawn_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<planet_materials::UnderMaterial>>,
) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::from_scale(Vec3::splat(DEFAULT_RADIUS as f32 * 2.0)),
        material: materials
            .add(planet_materials::UnderMaterial {
                pixels: 100.0,
                rotation: 0.0,
                light_origin: Vec2 { x: 0.39, y: 0.39 },
                time_speed: 0.2,
                dither_size: 2.0,
                light_border_1: 0.4,
                light_border_2: 0.6,
                color1: Color::rgb(1.0, 0.0, 0.0),
                color2: Color::rgb(0.0, 1.0, 0.0),
                color3: Color::rgb(0.0, 0.0, 1.0),
                size: 50.0,
                octaves: 4,
                seed: 14.0,
            })
            .clone(),
        ..default()
    });

    commands.spawn(PlanetBundle {
        ..Default::default()
    });
}

fn radius_to_mass(radius: u32) -> u32 {
    (PI * radius.pow(2) as f64) as u32
}
