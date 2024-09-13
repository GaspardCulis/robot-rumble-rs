use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use super::materials;

const DEFAULT_RADIUS: u32 = 128;

struct PlanetLayer<M: Material2d> {
    material: M,
    scale: f32,
    z_index: f32,
}

impl<M: Material2d> PlanetLayer<M> {
    fn instanciate(&self, world: &mut World) -> Entity {
        let material = world.resource_mut::<Assets<M>>().add(self.material.clone());
        let mesh = world
            .resource_mut::<Assets<Mesh>>()
            .add(Mesh::from(Rectangle::default()))
            .into();

        world
            .spawn(MaterialMesh2dBundle {
                mesh,
                transform: Transform::from_scale(Vec3::splat(DEFAULT_RADIUS as f32 * 2.0))
                    .with_translation(Vec3 {
                        x: 0.,
                        y: 0.,
                        z: self.z_index,
                    }),
                material,
                ..default()
            })
            .id()
    }
}

pub enum PlanetKind {
    EarthLike {
        ocean: PlanetLayer<materials::UnderMaterial>,
        land: PlanetLayer<materials::LandmassesMaterial>,
        clouds: PlanetLayer<materials::CloudsMaterial>,
    },
    MoonLike {
        under: PlanetLayer<materials::UnderMaterial>,
        craters: PlanetLayer<materials::CratersMaterial>,
    },
}

impl PlanetKind {
    pub fn instanciate(&self, world: &mut World) -> Vec<Entity> {
        match self {
            PlanetKind::EarthLike {
                ocean,
                land,
                clouds,
            } => vec![
                ocean.instanciate(world),
                land.instanciate(world),
                clouds.instanciate(world),
            ],
            PlanetKind::MoonLike { under, craters } => {
                vec![under.instanciate(world), craters.instanciate(world)]
            }
        }
    }
}
