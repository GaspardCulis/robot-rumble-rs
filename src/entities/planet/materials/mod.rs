use crate::network;

use super::Radius;
use bevy::{
    asset::{load_internal_asset, weak_handle},
    prelude::*,
    reflect::GetTypeRegistration,
    sprite::{Material2d, Material2dPlugin},
};
use rand::{Rng, SeedableRng as _};
use rand_xoshiro::Xoshiro256PlusPlus;
use std::marker::PhantomData;

mod clouds;
mod common;
mod craters;
mod dry_terrain;
mod gas_layers;
mod lakes;
mod landmasses;
mod ring;
mod star;
mod star_blobs;
mod star_flares;
mod under;
pub use clouds::CloudsMaterial;
pub use common::CommonMaterial;
pub use craters::CratersMaterial;
pub use dry_terrain::DryTerrainMaterial;
pub use gas_layers::GasLayersMaterial;
pub use lakes::LakesMaterial;
pub use landmasses::LandmassesMaterial;
pub use ring::RingMaterial;
pub use star::StarMaterial;
pub use star_blobs::StarBlobsMaterial;
pub use star_flares::StarFlaresMaterial;
pub use under::UnderMaterial;

pub struct PlanetMaterialsPlugin;

const PLANET_COMMON_HANDLE: Handle<Shader> = weak_handle!("7c4ab632-3073-479a-936d-dbe7e2d7ece7");

pub trait PlanetMaterial: Material2d + GetTypeRegistration {
    type Config: Component + Clone;

    fn from_config(
        common: CommonMaterial,
        config: &Self::Config,
        images: &mut ResMut<Assets<Image>>,
    ) -> Self;
}

impl Plugin for PlanetMaterialsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PLANET_COMMON_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/planet/common.wgsl"
            ),
            Shader::from_wgsl
        );

        app.add_plugins(PlanetMaterialPlugin::<CloudsMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<CratersMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<DryTerrainMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<GasLayersMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<LakesMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<LandmassesMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<RingMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<StarMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<StarBlobsMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<StarFlaresMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<UnderMaterial>::default());
    }
}

struct PlanetMaterialPlugin<M: PlanetMaterial>(PhantomData<M>);

impl<T: PlanetMaterial> Default for PlanetMaterialPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<M: PlanetMaterial> Plugin for PlanetMaterialPlugin<M>
where
    M::Data: PartialEq + Eq + core::hash::Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.register_type::<M>()
            .add_plugins(Material2dPlugin::<M>::default())
            .add_systems(
                Update,
                instance_layer_material::<M>.run_if(resource_exists::<network::SessionSeed>),
            );
    }
}

#[derive(Component)]
pub struct PlanetMaterialLayerInit<M: PlanetMaterial> {
    pub config: M::Config,
    pub scale: f32,
    pub z_index: f32,
}

fn instance_layer_material<M: PlanetMaterial>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut material: ResMut<Assets<M>>,
    query: Query<(Entity, &Radius, &PlanetMaterialLayerInit<M>), Added<PlanetMaterialLayerInit<M>>>,
    seed: Res<network::SessionSeed>,
) {
    for (entity, radius, layer) in query.iter() {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed.0);

        let common = CommonMaterial {
            pixels: f32::min(radius.0 as f32 / 2., 200.),
            seed: rng.random(),
            ..Default::default()
        }
        .scale(layer.scale);

        let mesh_bundle_entity = commands
            .spawn((
                Name::new(M::short_type_path()),
                Mesh2d(meshes.add(Mesh::from(Rectangle::default()))),
                MeshMaterial2d(material.add(M::from_config(common, &layer.config, &mut images))),
                Transform::from_scale(Vec3::splat((radius.0 as f32 * 2.0) * layer.scale))
                    .with_translation(Vec3 {
                        x: 0.,
                        y: 0.,
                        z: layer.z_index,
                    }),
            ))
            .id();

        commands
            .entity(entity)
            .add_children(&[mesh_bundle_entity])
            .remove::<PlanetMaterialLayerInit<M>>();
    }
}
