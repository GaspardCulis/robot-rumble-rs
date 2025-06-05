use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::{
    GameState,
    core::{
        physics::{PhysicsSet, Position},
        worldgen,
    },
    entities::player::{Player, weapon::WeaponType},
};

#[derive(Resource, Reflect)]
pub struct MapLimit {
    // TODO: Reconsider using existing Radius component
    pub radius: f32,
    /// Precomputed for performance optimizations
    radius_squared: f32,
}

#[derive(Event)]
#[allow(dead_code)] // Temporarly until entity gets used
/// Points to a Player entity
pub struct DeathEvent(pub Entity);

pub struct MapLimitPlugin;
impl Plugin for MapLimitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MapLimit>()
            .add_event::<DeathEvent>()
            .add_systems(
                Update,
                setup.run_if(resource_added::<worldgen::WorldgenAssets>),
            )
            .add_systems(
                GgrsSchedule,
                check_outsiders
                    .in_set(PhysicsSet::Collision)
                    .run_if(in_state(GameState::InGame).and(resource_exists::<MapLimit>)),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    worldgen_assets: Res<worldgen::WorldgenAssets>,
    worldgen_configs: Res<Assets<worldgen::WorldgenConfig>>,
) {
    let Some(worldgen_config) = worldgen_configs.get(&worldgen_assets.config) else {
        warn!("Worldgen config not loaded yet");
        return;
    };

    let limit = MapLimit {
        radius: worldgen_config.edge_radius as f32,
        radius_squared: worldgen_config.edge_radius.pow(2) as f32,
    };

    let edge_color = Srgba::hex("#00F9DE").unwrap();
    commands.spawn((
        Mesh2d(meshes.add(Mesh::from(Annulus::new(
            limit.radius - 2.0,
            limit.radius + 2.0,
        )))),
        MeshMaterial2d(color_materials.add(ColorMaterial::from_color(edge_color))),
        Transform::default(),
    ));

    commands.insert_resource(limit);
}

// FIX: Ugly AF
fn check_outsiders(
    mut commands: Commands,
    mut death_events: EventWriter<DeathEvent>,
    query: Query<(Entity, &Position, Has<Player>, Has<WeaponType>)>,
    limit: Res<MapLimit>,
) {
    for (entity, position, is_player, is_weapon) in query.iter() {
        if position.length_squared() > limit.radius_squared {
            if is_weapon {
                continue;
            } else if is_player {
                death_events.write(DeathEvent(entity));

                // FIX: Temporary way to handle death
                commands.entity(entity).remove::<Position>();
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
