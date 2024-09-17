use std::marker::PhantomData;

use bevy::{
    asset::load_internal_asset,
    prelude::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

mod clouds;
mod common;
mod craters;
mod landmasses;
mod under;
pub use clouds::CloudsMaterial;
pub use common::CommonMaterial;
pub use craters::CratersMaterial;
pub use landmasses::LandmassesMaterial;
pub use under::UnderMaterial;

pub struct PlanetMaterialsPlugin;

const PLANET_COMMON_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0xF750100345124C4BA08A7406DD1CFEC1);

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
            .add_plugins(PlanetMaterialPlugin::<LandmassesMaterial>::default())
            .add_plugins(PlanetMaterialPlugin::<UnderMaterial>::default());
    }
}

#[derive(Default)]
struct PlanetMaterialPlugin<M: Material2d>(PhantomData<M>);

impl<M: Material2d> Plugin for PlanetMaterialPlugin<M>
where
    M::Data: PartialEq + Eq + core::hash::Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<M>::default())
            .add_systems(Update, instance_layer_material::<M>);
    }
}

#[derive(Component)]
pub struct PlanetMaterialLayerInit<M: Material2d> {
    pub material: M,
    pub scale: f32,
    pub z_index: f32,
}

fn instance_layer_material<M: Material2d>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<M>>,
    query: Query<(Entity, &PlanetMaterialLayerInit<M>)>,
) {
    for (entity, layer) in query.iter() {
        let mesh_bundle_entity = commands
            .spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
                transform: Transform::from_scale(Vec3::splat(layer.scale)).with_translation(Vec3 {
                    x: 0.,
                    y: 0.,
                    z: layer.z_index,
                }),
                material: material.add(layer.material.clone()),
                ..default()
            })
            .id();

        commands
            .entity(entity)
            .push_children(&[mesh_bundle_entity])
            .remove::<PlanetMaterialLayerInit<M>>();
    }
}
