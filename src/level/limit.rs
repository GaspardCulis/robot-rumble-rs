use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

use crate::{
    GameState,
    core::{
        physics::{PhysicsSet, Position},
        worldgen,
    },
    entities::player::{
        Percentage, Player,
        inventory::Arsenal,
        weapon::config::{WeaponStats, WeaponType},
    },
};

#[derive(Resource, Reflect)]
pub struct MapLimit {
    // TODO: Reconsider using existing Radius component
    pub radius: f32,
    /// Precomputed for performance optimizations
    radius_squared: f32,
}

#[derive(Event)]
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
                (check_outsiders, handle_player_death)
                    .chain()
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

pub fn handle_player_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    query: Query<&Arsenal, With<Player>>,
) -> Result {
    for DeathEvent(player) in death_events.read() {
        // Refresh player
        commands
            .entity(*player)
            .remove::<Position>()
            .insert(Percentage::default());
        // Refresh weapons
        let arsenal = query.get(*player)?;
        for (_, weapon) in arsenal.0.iter() {
            warn!("Removing weapon stats!");
            commands.entity(*weapon).remove::<WeaponStats>();
        }
    }
    Ok(())
}

fn check_outsiders(
    mut commands: Commands,
    mut death_events: EventWriter<DeathEvent>,
    query: Query<(Entity, &Position, Has<Player>), Without<WeaponType>>,
    limit: Res<MapLimit>,
) {
    for (entity, position, is_player) in query.iter() {
        if position.length_squared() > limit.radius_squared {
            if is_player {
                death_events.write(DeathEvent(entity));
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
